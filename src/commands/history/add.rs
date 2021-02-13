use structopt::{clap, StructOpt};

use crate::types::{CliResult, CommandResult};

#[derive(Debug, StructOpt)]
#[structopt(name = "add", about = "add history to bookmark")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct HistoryAdd {}

impl HistoryAdd {
    pub fn run(&self) -> CliResult {
        debug!("{:?}", self);
        Ok(CommandResult::DisplayNone)
    }
}
