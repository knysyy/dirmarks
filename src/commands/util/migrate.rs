use std::{fs::File, io, path::PathBuf};

use structopt::{clap, StructOpt};

use crate::{
    commands::Command,
    models::{bookmark, history},
    types::{CliResult, CommandError, CommandResult},
    utils::{config::CONFIG, database::establish_connection},
};

#[derive(Debug, StructOpt)]
#[structopt(name = "migration", about = "migration")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Migrate {}

impl Command for Migrate {
    fn execute(&self) -> CliResult {
        let path = PathBuf::from(&CONFIG.database_url);

        if !path.exists() {
            match File::create(path) {
                Ok(_) => {},
                Err(err) => {
                    let err_kind = err.kind();
                    if err_kind != io::ErrorKind::AlreadyExists {
                        return Err(CommandError::IoError(err));
                    }
                },
            }
        }

        let conn = &mut establish_connection()?;

        bookmark::create_bookmarks_table(conn)?;
        history::create_histories_table(conn)?;

        Ok(CommandResult::Migrated(CONFIG.database_url.clone()))
    }
}
