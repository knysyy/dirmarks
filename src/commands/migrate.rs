use std::{env, fs::File, io, path::PathBuf};

use structopt::{clap, StructOpt};

use crate::{
    errors::CommandError,
    models::{bookmark, history, result::CommandResult},
    types::CliResult,
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "migration", about = "migration")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Migrate {}

impl Migrate {
    pub fn run(&self) -> CliResult {
        let database_url = env::var("DM_DATABASE_URL").unwrap_or("~/bd.db".to_string());
        let path = PathBuf::from(&database_url);

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

        let conn = establish_connection()?;

        bookmark::create_bookmarks_table(&conn)?;
        history::create_histories_table(&conn)?;

        Ok(CommandResult::Migrated(database_url.to_string()))
    }
}
