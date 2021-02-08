use structopt::{clap, StructOpt};

use crate::{
    database::{connection::establish_connection, repository},
    error::CommandError,
    result::CommandResult,
    types::CliResult,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "del", about = "delete directory bookmrak")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Delete {
    #[structopt(index = 1)]
    key: String,
}

impl Delete {
    pub fn run(&self) -> CliResult {
        let conn = establish_connection()?;
        match repository::get_bookmark(&conn, &self.key) {
            Ok(bookmark) => {
                repository::delete_bookmark(&conn, &self.key)?;
                Ok(CommandResult::Deleted(bookmark.key, bookmark.path))
            },
            Err(diesel::NotFound) => Err(CommandError::NotFound),
            Err(err) => Err(CommandError::Database(err)),
        }
    }
}
