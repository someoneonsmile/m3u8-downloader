use console::{style, Emoji};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use log::*;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::task::JoinHandle;

mod cli;

type Result<O> = anyhow::Result<O>;

static PREFIX_EMOJI: Emoji<'_, '_> = Emoji("🛸🚀", "");

#[tokio::main]
async fn main() -> Result<()> {
    let started = Instant::now();

    env_logger::Builder::from_default_env()
        .parse_filters("info")
        .default_format()
        .format_level(true)
        .format_target(false)
        .format_module_path(false)
        .format_timestamp(None)
        .init();

    let opt = cli::Opt::parse();
    debug!("opt: {:?}", opt);

    let url = opt.url.as_str();
    let base_url = url.split_at(url.rfind('/').expect("please input the m3u8 url")).0;

    let tmp_dir = tokio::task::spawn_blocking(move || {
        tempfile::Builder::new().prefix("m3u8-downloader").tempdir()
    })
    .await??;
    let tmp_list_path = "ts_list.txt";

    let client = reqwest::Client::new();
    let ts_list_str = client.get(url).send().await?.text().await?;
    let ts_list: Vec<&str> = ts_list_str
        .lines()
        .filter(|line| !line.is_empty() && !line.trim_start().starts_with('#'))
        .collect();

    // 生成文件清单文件
    let content = ts_list
        .iter()
        .map(|line| format!("file {}\n", line))
        .collect::<String>();
    let mut tmp_list_file = fs::File::create(tmp_dir.path().join(tmp_list_path)).await?;
    tmp_list_file.write_all(content.as_bytes()).await?;

    // 进度条
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠁⠂⠄⡀⡀⢀⠠⠐⠈⠈")
        .template(&format!(
            "{}",
            style(format!(
                "{{prefix:.bold}} {{spinner}} {}: {{wide_msg}}",
                PREFIX_EMOJI
            ))
            .bold()
            .yellow()
        ));
    let progress_bar = ProgressBar::new(ts_list.len() as u64);
    progress_bar.set_style(spinner_style);

    // 下载 ts_list
    let mut handles = Vec::<JoinHandle<Result<()>>>::new();
    let client = Arc::new(client);
    for ts in ts_list {
        let client = client.clone();
        let progress_bar = progress_bar.clone();
        let mut ts = ts.to_owned().clone();
        ts = format!("{}/{}", base_url, ts);
        handles.push(tokio::spawn(async move {
            download_file(&client, &ts, "").await?;
            progress_bar.inc(1);
            Ok(()) as Result<()>
        }));
    }
    for handle in handles {
        handle.await??;
    }

    // 调用 ffmpeg 合并文件
    // command = 'ffmpeg -y -f concat -i %s -bsf:a aac_adtstoasc -c copy %s' % (concatfile, path)
    Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-i")
        .arg(tmp_list_path)
        .arg("-bsf:a")
        .arg("aac_adtstoasc")
        .arg("-c")
        .arg("copy")
        .arg(opt.dest)
        .current_dir(tmp_dir)
        .spawn()?
        .wait()
        .await?;

    info!(
        "take: {}",
        HumanDuration(Instant::now().duration_since(started))
    );
    Ok(())
}

async fn download_file<P>(client: &reqwest::Client, url: &str, dest: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let content = client.get(url).send().await?.text().await?;
    let mut dest = fs::File::create(dest).await?;
    dest.write_all(content.as_bytes()).await?;
    Ok(())
}
