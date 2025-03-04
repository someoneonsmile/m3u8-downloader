use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use url::Url;

use crate::{cli::Cli, constants::MAX_PARALLEL_DOWNLOAD};

pub struct Opt {
    /// dest path
    pub dest: PathBuf,
    /// parallel worker num
    pub worker: usize,
    /// `source_file` used for from file
    pub source: Option<PathBuf>,
    /// url to download
    pub url: Option<Url>,
}

impl TryFrom<Cli> for Opt {
    type Error = anyhow::Error;
    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        Ok(Self {
            dest: cli.dest,
            worker: std::cmp::min(cli.worker, MAX_PARALLEL_DOWNLOAD),
            source: cli.source.source,
            url: cli.source.url.map(|s| Url::parse(&s)).transpose()?,
        })
    }
}

impl Opt {
    pub fn from_cli() -> Result<Opt> {
        let cli = Cli::try_parse()?;
        let opt = cli.try_into()?;
        Ok(opt)
    }
}
