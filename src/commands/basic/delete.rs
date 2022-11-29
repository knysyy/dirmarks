use log::debug;
use structopt::{clap, StructOpt};

use crate::{
    models::bookmark,
    types::{CliResult, CommandError, CommandResult},
    utils::database::establish_connection,
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
        debug!("{:?}", self);
        let conn = &mut establish_connection()?;
        match bookmark::get_bookmark_by_key(conn, &self.key) {
            Ok(bookmark) => {
                bookmark::delete_bookmark(conn, &self.key)?;
                Ok(CommandResult::Deleted(bookmark.key, bookmark.path))
            },
            Err(diesel::NotFound) => Err(CommandError::KeyNotFoundError(self.key.clone())),
            Err(err) => Err(CommandError::DieselError(err)),
        }
    }
}
