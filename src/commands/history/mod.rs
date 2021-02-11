use structopt::{clap, StructOpt};

use crate::types::CliResult;

mod add;

#[derive(Debug, StructOpt)]
#[structopt(name = "history", about = "manipulate history")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub enum History {
    Add(add::HistoryAdd),
}

impl History {
    pub fn run(&self) -> CliResult {
        match self {
            History::Add(history_add) => history_add.run(),
        }
    }
}
