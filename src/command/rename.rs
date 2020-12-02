use structopt::{clap, StructOpt};

use crate::database::connection::establish_connection;
use crate::database::repository;
use crate::types::CliResult;
use crate::error::CommandError;
use crate::result::CommandResult;

#[derive(Debug, StructOpt)]
#[structopt(name = "rename", about = "rename directory bookmark key")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Rename {
    #[structopt(short = "o", long = "old-key")]
    old_key: String,

    #[structopt(short = "n", long = "new-key")]
    new_key: String,
}

impl Rename {
    pub fn run(&self) -> CliResult {
        let conn = establish_connection()?;
        match repository::get_bookmark(&conn, &self.old_key) {
            Ok(_) => {
                repository::rename_bookmark(&conn, &self.old_key, &self.new_key)?;
                Ok(CommandResult::Renamed(self.old_key.to_string(), self.new_key.to_string()))
            }
            Err(diesel::NotFound) => {
                Err(CommandError::NotFound)
            }
            Err(err) => {
                Err(CommandError::Database(err))
            }
        }
    }
}
