use structopt::{clap, StructOpt};

use crate::types::CliResult;

mod jump;
mod save;

#[derive(Debug, StructOpt)]
#[structopt(name = "history", about = "manipulate history")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub enum History {
    Save(save::HistorySave),
    Jump(jump::HistoryJump),
}

impl History {
    pub fn run(&self) -> CliResult {
        match self {
            History::Save(history_save) => history_save.run(),
            History::Jump(history_add) => history_add.run(),
        }
    }
}
