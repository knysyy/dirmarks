use std::env;

use diesel::SqliteConnection;
use structopt::{clap, StructOpt};

use crate::{
    models::bookmark,
    types::{CliResult, CommandError, CommandResult},
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "add", about = "save directory")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Add {
    #[structopt(index = 1)]
    key: String,

    #[structopt(short, long)]
    path: Option<String>,

    #[structopt(short, long)]
    description: Option<String>,
}

impl Add {
    pub fn run(&self) -> CliResult {
        debug!("{:?}", self);
        let conn = establish_connection()?;
        match bookmark::get_bookmark(&conn, &self.key) {
            Ok(_) => Err(CommandError::KeyAlreadyExistError(self.key.clone())),
            Err(diesel::NotFound) => match &self.path {
                Some(path) => self.add_path_to_bookmark(&conn, &path),
                None => self.add_current_path_to_bookmark(&conn),
            },
            Err(err) => Err(CommandError::DieselError(err)),
        }
    }

    fn add_path_to_bookmark(&self, conn: &SqliteConnection, path: &str) -> CliResult {
        bookmark::create_bookmark(conn, &self.key, &path, self.description.as_deref())?;
        Ok(CommandResult::Added(self.key.to_string(), path.to_string()))
    }

    fn add_current_path_to_bookmark(&self, conn: &SqliteConnection) -> CliResult {
        match env::current_dir() {
            Ok(current_dir) => {
                let path = current_dir.to_str().unwrap();
                debug!("{}", path);
                bookmark::create_bookmark(conn, &self.key, path, self.description.as_deref())?;
                Ok(CommandResult::Added(self.key.to_string(), path.to_string()))
            },
            Err(err) => Err(CommandError::IoError(err)),
        }
    }
}
