use std::io::Write;

use structopt::{clap, StructOpt};

use crate::{
    fzf::Fzf,
    models::history,
    types::{CliResult, CommandResult},
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "add", about = "add history to bookmark")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct HistoryJump {}

impl HistoryJump {
    pub fn run(&self) -> CliResult {
        debug!("{:?}", self);
        let conn = establish_connection()?;
        let histories = history::get_histories(&conn)?;
        let mut fzf = Fzf::new()?;
        let handle = fzf.stdin();
        for history in &histories {
            write!(handle, "{}", history.path)?;
        }
        let selection = fzf.wait_select()?;
        Ok(CommandResult::Jump(selection))
    }
}
