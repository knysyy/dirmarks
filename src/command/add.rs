use std::env;

use diesel::SqliteConnection;
use structopt::{clap, StructOpt};

use crate::CliResult;
use crate::database::connection::establish_connection;
use crate::database::repository::{create_bookmark, get_bookmark};
use crate::error::CommandError;
use crate::result::CommandResult;

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
        let conn = establish_connection()?;
        match get_bookmark(&conn, &self.key) {
            Ok(_) => {
                Err(CommandError::AlreadyExist)
            }
            Err(diesel::NotFound) => {
                match &self.path {
                    Some(path) => {
                        self.add_path_to_bookmark(&conn, &path)
                    }
                    None => {
                        self.add_current_path_to_bookmark(&conn)
                    }
                }
            }
            Err(err) => {
                Err(CommandError::Database(err))
            }
        }
    }

    fn add_path_to_bookmark(&self, conn: &SqliteConnection, path: &str) -> CliResult {
        create_bookmark(conn, &self.key, &path, self.description.as_deref())?;
        Ok(CommandResult::Added(self.key.to_string(), path.to_string()))
    }

    fn add_current_path_to_bookmark(&self, conn: &SqliteConnection) -> CliResult {
        match env::current_dir() {
            Ok(current_dir) => {
                let path = current_dir.to_str().unwrap();
                create_bookmark(conn, &self.key, path, self.description.as_deref())?;
                Ok(CommandResult::Added(self.key.to_string(), path.to_string()))
            }
            Err(err) => Err(CommandError::IoError(err))
        }
    }
}
