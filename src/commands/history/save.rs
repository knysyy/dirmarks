use std::env;

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
        let conn = establish_connection()?;
        match env::current_dir() {
            Ok(current_dir) => {
                let path = current_dir.to_str().unwrap();
                history::create_histories(&conn, path)?;
                Ok(CommandResult::DisplayNone)
            },
            Err(err) => Err(CommandError::IoError(err)),
        }
    }
}
