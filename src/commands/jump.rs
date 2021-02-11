use structopt::{clap, StructOpt};

use crate::{
    errors::CommandError,
    models::{bookmark, result::CommandResult},
    types::CliResult,
    utils::database::establish_connection,
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
        match bookmark::get_bookmark(&conn, &self.key) {
            Ok(bookmark) => Ok(CommandResult::Jump(bookmark.path)),
            Err(diesel::NotFound) => Err(CommandError::KeyNotFoundError(self.key.clone())),
            Err(err) => Err(CommandError::DieselError(err)),
        }
    }
}
