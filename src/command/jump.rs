use structopt::{clap, StructOpt};

use crate::{
    database::{connection::establish_connection, repository::get_bookmark},
    error::CommandError,
    result::CommandResult,
    types::CliResult,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "jump", about = "jump to directory bookmark")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Jump {
    #[structopt(index = 1)]
    key: String,
}

impl Jump {
    pub fn run(&self) -> CliResult {
        let conn = establish_connection()?;
        match get_bookmark(&conn, &self.key) {
            Ok(bookmark) => Ok(CommandResult::Jump(bookmark.path)),
            Err(diesel::NotFound) => Err(CommandError::NotFound),
            Err(err) => Err(CommandError::Database(err)),
        }
    }
}
