use structopt::{clap, StructOpt};

use crate::{
    errors::CommandError,
    models::result::CommandResult,
    types::CliResult,
    utils::{bookmark_service, database::establish_connection},
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
        match bookmark_service::get_bookmark(&conn, &self.key) {
            Ok(bookmark) => {
                bookmark_service::delete_bookmark(&conn, &self.key)?;
                Ok(CommandResult::Deleted(bookmark.key, bookmark.path))
            },
            Err(diesel::NotFound) => Err(CommandError::KeyNotFoundError(self.key.clone())),
            Err(err) => Err(CommandError::DieselError(err)),
        }
    }
}
