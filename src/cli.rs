use std::path::PathBuf;

use structopt::StructOpt;

/// m3u8 downloader
#[derive(StructOpt, Debug)]
#[structopt(name = "m3u8-downloader")]
pub struct Opt {
    /// url to download
    #[structopt(long = "url")]
    pub url: String,
    /// dest path
    #[structopt(short = "d", long = "dest")]
    pub dest: PathBuf,
}

impl Opt {
    pub fn parse() -> Opt {
        Opt::from_args()
    }
}
