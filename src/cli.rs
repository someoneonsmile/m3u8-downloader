use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// dest path
    #[arg(short, long)]
    pub dest: PathBuf,
    /// parallel worker num
    #[arg(short, long, default_value = "20")]
    pub worker: usize,

    #[command(flatten)]
    pub source: Sources,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct Sources {
    /// source mode used for from file
    /// when `--file` file content segement's uri is not absolute
    /// should add the `--url` option
    #[arg(short, long)]
    pub source: Option<PathBuf>,
    /// url mode used for from url
    #[arg(long)]
    pub url: Option<String>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
