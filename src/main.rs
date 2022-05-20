use clap::Parser;
use console::Emoji;
use futures::stream::{StreamExt, TryStreamExt};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

mod cli;

type Result<Output> = anyhow::Result<Output>;

/// prefix emoji
static PREFIX_EMOJIS: [Emoji<'_, '_>; 4] = [
    Emoji("🛸", ""),
    Emoji("🚀", ""),
    Emoji("🛴", ""),
    Emoji("🛹", ""),
];

/// 最大同时下载数
static MAX_PARALLEL_DOWNLOAD: usize = 50;

#[tokio::main]
async fn main() -> Result<()> {
    let started = Instant::now();

    // TODO: replace with color_eyon, tracing
    // env_logger::Builder::from_default_env()
    //     .parse_filters("info")
    //     .default_format()
    //     .format_level(true)
    //     .format_target(false)
    //     .format_module_path(false)
    //     .format_timestamp(None)
    //     .init();

    let mut opt = cli::Opt::parse();
    opt.worker = std::cmp::min(opt.worker, MAX_PARALLEL_DOWNLOAD);
    let opt = opt;

    // reqwest client
    // DNS resolve with trust_dns
    let client = reqwest::ClientBuilder::new().trust_dns(true).build()?;

    let url = opt.url.as_str();
    let base_url = url
        .split_at(url.rfind('/').expect("please input the m3u8 url"))
        .0;

    // 生成临时下载目录
    let tmp_dir = tokio::task::spawn_blocking(move || {
        tempfile::Builder::new().prefix("m3u8-downloader").tempdir()
    })
    .await??;

    // 下载文件清单文件
    let ts_list_file_path = "ts_list.txt";
    let ts_list_str = client.get(url).send().await?.text().await?;
    let ts_list: Vec<&str> = ts_list_str
        .lines()
        .filter(|line| !line.is_empty() && !line.trim_start().starts_with('#'))
        .collect();
    let content = ts_list
        .iter()
        .map(|line| format!("file {}\n", line))
        .collect::<String>();
    let mut tmp_list_file = fs::File::create(tmp_dir.path().join(ts_list_file_path)).await?;
    tmp_list_file.write_all(content.as_bytes()).await?;

    // 进度条
    let main_bar = MultiProgress::new();

    let pb_style = ProgressStyle::with_template(
        "{spinner:.green} {prefix} [{elapsed_precise}] [{bar:60.cyan/blue}] [{bytes}/{total_bytes}] {binary_bytes_per_sec} ({eta})",
    )?
    .progress_chars("#>-");

    let mut emoji_iter = PREFIX_EMOJIS.iter().cycle();
    // TODO: 是否可以生成多个 tokio 任务, 在多个线程同时执行
    // 并发下载 ts_list
    let buffered = futures::stream::iter(ts_list)
        .map(|ts| {
            let client = client.clone();
            let ts_url = format!("{}/{}", base_url, ts);
            let ts_file_path = tmp_dir.path().join(ts);
            let pb = main_bar.add(ProgressBar::new(0));
            pb.set_style(pb_style.clone());
            pb.set_prefix(format!(
                "{} [downloading {}]",
                emoji_iter.next().unwrap(),
                ts
            ));
            async move { download_file(client, &ts_url, ts_file_path, pb).await }
        })
        .buffer_unordered(opt.worker);

    buffered.try_collect::<Vec<()>>().await?;

    // 合并视频
    merge_video(tmp_dir.path(), ts_list_file_path, opt.dest).await?;

    // 打印用时统计
    println!(
        "take: {}",
        HumanDuration(Instant::now().duration_since(started))
    );

    Ok(())
}

async fn download_file<P>(
    client: reqwest::Client,
    url: &str,
    dest: P,
    pb: ProgressBar,
) -> Result<()>
where
    P: AsRef<Path>,
{
    // https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d

    let response = client.get(url).send().await?;

    let total_size = response
        .content_length()
        .ok_or_else(|| anyhow::format_err!("can't get the content_length"))?;

    // 进度条长度
    pb.set_length(total_size);

    let mut dest = fs::File::create(dest).await?;
    let mut downloaded: u64 = 0;

    // 流下载
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        dest.write_all(&chunk).await?;
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        pb.set_position(downloaded);
    }

    pb.finish_and_clear();
    Ok(())
}

async fn merge_video<P1, P2>(tmp_dir: P1, ts_list_file_path: &str, dest: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<std::ffi::OsStr>,
{
    // 调用 ffmpeg 合并文件
    // command = 'ffmpeg -y -f concat -i %s -bsf:a aac_adtstoasc -c copy %s' % (concatfile, path)
    Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-i")
        .arg(ts_list_file_path)
        .arg("-bsf:a")
        .arg("aac_adtstoasc")
        .arg("-c")
        .arg("copy")
        .arg(dest)
        .current_dir(tmp_dir)
        .spawn()?
        .wait()
        .await?;
    Ok(())
}
