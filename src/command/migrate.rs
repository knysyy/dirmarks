use std::{env, fs::File, path::PathBuf};

use dotenv::dotenv;
use structopt::{clap, StructOpt};

use crate::{
    database::{
        connection::establish_connection, repository::create_bookmarks_table,
    },
    error::CommandError,
    result::CommandResult,
    types::CliResult,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "migration", about = "migration")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Migrate {}

impl Migrate {
    pub fn run(&self) -> CliResult {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").unwrap_or("~/bd.db".to_string());
        let path = PathBuf::from(&database_url);
        if path.exists() {
            return Err(CommandError::AlreadyFileExist);
        }
        File::create(path)?;
        let conn = establish_connection()?;
        create_bookmarks_table(&conn)?;
        Ok(CommandResult::Migrated(format!(
            "Database file created file : {}",
            &database_url
        )))
    }
}
