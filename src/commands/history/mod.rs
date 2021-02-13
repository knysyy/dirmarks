use structopt::{clap, StructOpt};

use crate::types::CliResult;

mod add;
mod save;

#[derive(Debug, StructOpt)]
#[structopt(name = "history", about = "manipulate history")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub enum History {
    Save(save::HistorySave),
    Add(add::HistoryAdd),
}

impl History {
    pub fn run(&self) -> CliResult {
        match self {
            History::Save(history_save) => history_save.run(),
            History::Add(history_add) => history_add.run(),
        }
    }
}
