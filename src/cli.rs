use std::path::PathBuf;

use crate::constants::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, name = "m3u8-downloader")]
pub struct Opt {
    /// url to download
    #[clap(long)]
    pub url: String,
    /// dest path
    #[clap(short, long)]
    pub dest: PathBuf,
    /// parallel worker num
    #[clap(short, long, default_value = "20")]
    pub worker: usize,
}

impl Opt {
    pub(crate) fn get() -> Opt {
        let mut opt = Opt::parse();
        opt.worker = std::cmp::min(opt.worker, MAX_PARALLEL_DOWNLOAD);
        opt
    }
}
