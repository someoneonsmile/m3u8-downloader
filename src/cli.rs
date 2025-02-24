use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(arg_required_else_help = true)]
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
