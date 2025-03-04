use anyhow::anyhow;
use console::Emoji;
use directories::ProjectDirs;
use futures::stream::{StreamExt, TryStreamExt};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use opt::Opt;
use reqwest::IntoUrl;
use std::fmt::Write;
use std::path::Path;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use url::Url;

use crate::constants::{PREFIX_EMOJIS, TS_LIST_PATH};

mod cli;
mod constants;
mod m3u8;
mod opt;
mod request;
mod util;

type Result<Output> = anyhow::Result<Output>;

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

    let opt = Opt::from_cli()?;

    let m3u8_content = get_m3u8_content(&opt).await?;

    // TODO: use builder
    // 生成临时下载目录
    let tmp_dir = make_sure_url_dir(&m3u8_content).await?;

    // 下载文件清单文件
    let ts_list_abs_path = tmp_dir.as_ref().join(TS_LIST_PATH);

    // let ts_list_cotent_origin = request::get_bytes(m3u8_url.clone()).await?;
    let media_playlist = m3u8::parse(opt.url.as_ref(), &m3u8_content).await?;
    let segements_uri_list: Vec<Url> = media_playlist
        .segments
        .iter()
        .map(|s| {
            let uri = &s.uri;
            if let Ok(uri) = Url::parse(uri) {
                anyhow::Ok(uri)
            } else {
                let base_url = opt.url.as_ref().ok_or_else(|| {
                    anyhow!(
                        "the url in the m3u8 file is not absolute, please support the url option"
                    )
                })?;
                let uri = base_url.join(uri)?;
                anyhow::Ok(uri)
            }
        })
        .collect::<Result<Vec<Url>>>()?;
    let ts_list: Vec<&str> = segements_uri_list
        .iter()
        .map(|uri| {
            let last_path = uri
                .path_segments()
                .and_then(Iterator::last)
                .ok_or_else(|| anyhow!("can't get segments uri, {uri:?}"))?;
            anyhow::Ok(last_path)
        })
        .collect::<Result<Vec<&str>>>()?;
    let ts_list_file_content = ts_list.iter().fold(String::new(), |mut buf, line| {
        let _ = writeln!(buf, "file {line}");
        buf
    });

    let mut tmp_list_file = fs::File::create(ts_list_abs_path).await?;
    tmp_list_file
        .write_all(ts_list_file_content.as_bytes())
        .await?;

    // 进度条
    let main_bar = MultiProgress::new();

    // 进度条样式
    let pb_style = ProgressStyle::with_template(
        "{spinner:.green} {prefix} [{elapsed_precise}] [{bar:60.cyan/blue}] [{bytes}/{total_bytes}] {binary_bytes_per_sec} ({eta})",
    )?
    .progress_chars("#>-");

    // TODO: 是否可以生成多个 tokio 任务, 在多个线程同时执行
    // 并发下载 ts_list
    let mut emoji_iter = PREFIX_EMOJIS.iter().cycle();
    let total = ts_list.len();

    // ----------------------------------------------------------------------
    //    - 并发下载, 方式一 (for_each_concurrent) -
    // ----------------------------------------------------------------------

    futures::stream::iter(segements_uri_list.iter().zip(ts_list.iter()))
        .enumerate()
        .map(Ok)
        .try_for_each_concurrent(opt.worker, |(index, (ts_url, ts_name))| {
            let ts_file_path = tmp_dir.as_ref().join(ts_name);
            let pb = main_bar.add(ProgressBar::new(0));
            pb.set_style(pb_style.clone());
            pb.set_prefix(format!(
                "{} [{}/{}] [{}]",
                emoji_iter.next().unwrap_or(&Emoji("", "")),
                index,
                total,
                ts_name
            ));
            async move {
                download_file(ts_url.to_owned(), ts_file_path, pb).await?;
                Ok(()) as Result<()>
            }
        })
        .await?;

    // ----------------------------------------------------------------------
    //    - 并发下载, 方式二 (普通 iter collect to FutureUnordered) -
    // ----------------------------------------------------------------------

    // ----------------------------------------------------------------------
    //    - 并发下载, 方式三 (map to future then buffer_unordered) -
    // ----------------------------------------------------------------------

    // futures::stream::iter(ts_list)
    //     .enumerate()
    //     .map(|(index, ts)| {
    //         let client = client.clone();
    //         let ts_url = format!("{}/{}", base_url, ts);
    //         let ts_file_path = tmp_dir.as_ref().join(ts);
    //         let pb = main_bar.add(ProgressBar::new(0));
    //         pb.set_style(pb_style.clone());
    //         pb.set_prefix(format!(
    //             "{} [{}/{}] [{}]",
    //             emoji_iter.next().unwrap(),
    //             index,
    //             total,
    //             ts
    //         ));
    //         async move {
    //             download_file(client, &ts_url, ts_file_path, pb).await?;
    //             Ok(()) as Result<()>
    //         }
    //     })
    //     .buffer_unordered(opt.worker)
    //     .try_collect::<Vec<()>>()
    //     .await?;

    // 合并视频
    merge_video(tmp_dir.as_ref(), opt.dest).await?;

    // 删除临时文件
    fs::remove_dir_all(tmp_dir).await?;

    // 打印用时统计
    println!(
        "take: {}",
        HumanDuration(Instant::now().duration_since(started))
    );

    Ok(())
}

async fn get_m3u8_content(opt: &Opt) -> Result<Vec<u8>> {
    if let Some(url) = &opt.url {
        let content = request::get_bytes(url.clone()).await?;
        Ok(content)
    } else if let Some(path) = &opt.source {
        let content = fs::read(path).await?;
        Ok(content)
    } else {
        unreachable!("must set url or source_file option")
    }
}

async fn download_file<U, P>(url: U, dest: P, pb: ProgressBar) -> Result<()>
where
    U: IntoUrl,
    P: AsRef<Path>,
{
    if dest.as_ref().exists() {
        pb.finish_and_clear();
        return Ok(());
    }
    // https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
    let response = request::get(url).await?;

    let total_size = response
        .content_length()
        .ok_or_else(|| anyhow::format_err!("can't get the content_length"))?;

    // 进度条长度
    pb.set_length(total_size);

    let part_path = format!(
        "{}{}",
        dest.as_ref()
            .to_str()
            .ok_or_else(|| anyhow::format_err!("dest({:?}) to_str error", dest.as_ref()))?,
        ".part"
    );
    let mut part = fs::File::create(&part_path).await?;
    let mut downloaded: u64 = 0;

    // 流下载
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        part.write_all(&chunk).await?;
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        pb.set_position(downloaded);
    }

    fs::rename(part_path, dest).await?;
    pb.finish_and_clear();
    Ok(())
}

async fn merge_video<P1, P2>(tmp_dir: P1, dest: P2) -> Result<()>
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
        .arg(TS_LIST_PATH)
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

async fn make_sure_url_dir(bytes: &[u8]) -> Result<impl AsRef<Path>> {
    // url hash
    let hash = util::hash(&bytes);

    // create cache_dir
    let url_dir = ProjectDirs::from("", "", "m3u8-downloader")
        .ok_or_else(|| anyhow::anyhow!("not find ProjectDirs"))?
        .cache_dir()
        .join(hash.to_string());
    println!("use cache dir: {}", url_dir.to_string_lossy());
    make_sure_dir_exsit(url_dir).await
}

// AsRef::<Path>::as_ref("/tmp");
// async fn make_sure_dir_exsit(path: impl AsRef<Path>) -> Result<impl AsRef<Path>> {
async fn make_sure_dir_exsit<P: AsRef<Path>>(path: P) -> Result<impl AsRef<Path>> {
    if path.as_ref().exists() {
        return Ok(path);
    }
    fs::create_dir_all(path.as_ref()).await?;
    Ok(path)
}
