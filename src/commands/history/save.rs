use std::env;

use anyhow::Context;
use log::debug;
use structopt::{clap, StructOpt};

use crate::{
    models::history,
    types::{CliResult, CommandError, CommandResult},
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "save", about = "save history")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct HistorySave {}

impl HistorySave {
    pub fn run(&self) -> CliResult {
        debug!("{:?}", self);
        let conn = &mut establish_connection()?;
        let current_dir = env::current_dir().context("Failed to get the current directory.")?;
        let path = current_dir
            .to_str()
            .context("Failed to convert current directory to path.")?;
        match history::get_history(conn, &path) {
            Ok(history) => {
                history::update_count(conn, &history.path, history.count + 1)?;
            },
            Err(diesel::NotFound) => {
                history::create_histories(conn, path)?;
            },
            Err(err) => {
                return Err(CommandError::DieselError(err));
            },
        }
        Ok(CommandResult::DisplayNone)
    }
}
