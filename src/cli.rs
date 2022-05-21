use std::{path::PathBuf, usize};

use clap::Parser;

/// m3u8 downloader
#[derive(Parser, Debug)]
#[clap(author, version, about, name = "m3u8-downloader")]
pub struct Opt {
    /// url to download
    #[clap(long)]
    pub url: String,
    /// dest path
    #[clap(short, long)]
    pub dest: PathBuf,
    /// dest path
    #[clap(short, long, default_value = "20")]
    pub worker: usize,
}