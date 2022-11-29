use log::debug;
use structopt::{clap, StructOpt};

use crate::{
    commands::Command,
    models::bookmark,
    types::{CliResult, CommandError, CommandResult},
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "jump", about = "jump to directory bookmark")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Jump {
    #[structopt(index = 1)]
    key: String,
}

impl Command for Jump {
    fn execute(&self) -> CliResult {
        debug!("{:?}", self);
        let conn = &mut establish_connection()?;
        match bookmark::get_bookmark_by_key(conn, &self.key) {
            Ok(bookmark) => Ok(CommandResult::Jump(bookmark.path)),
            Err(diesel::NotFound) => Err(CommandError::KeyNotFoundError(self.key.clone())),
            Err(err) => Err(CommandError::DieselError(err)),
        }
    }
}
