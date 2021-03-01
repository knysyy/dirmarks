use structopt::{clap, StructOpt};

use crate::{
    models::bookmark,
    types::{CliResult, CommandError, CommandResult},
    utils::database::establish_connection,
};

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
        debug!("{:?}", self);
        let conn = establish_connection()?;
        match bookmark::get_bookmark_by_key(&conn, &self.new_key) {
            Ok(_) => Err(CommandError::KeyAlreadyExistError(self.new_key.clone())),
            Err(diesel::NotFound) => match bookmark::get_bookmark_by_key(&conn, &self.old_key) {
                Ok(_) => {
                    bookmark::rename_bookmark(&conn, &self.old_key, &self.new_key)?;
                    Ok(CommandResult::Renamed(
                        self.old_key.to_string(),
                        self.new_key.to_string(),
                    ))
                },
                Err(diesel::NotFound) => Err(CommandError::KeyNotFoundError(self.old_key.clone())),
                Err(err) => Err(CommandError::DieselError(err)),
            },
            Err(err) => Err(CommandError::DieselError(err)),
        }
    }
}
