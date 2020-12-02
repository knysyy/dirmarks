use std::env;
use std::fs::File;
use std::path::PathBuf;

use structopt::{clap, StructOpt};

use dotenv::dotenv;

use crate::CliResult;
use crate::database::connection::establish_connection;
use crate::database::repository::create_bookmarks_table;
use crate::error::CommandError;
use crate::result::CommandResult;

#[derive(Debug, StructOpt)]
#[structopt(name = "migration", about = "migration")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Migrate {
    #[structopt(short = "o", long = "old-key")]
    old_key: String,

    #[structopt(short = "n", long = "new-key")]
    new_key: String,
}

impl Migrate {
    pub fn run(&self) -> CliResult {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .unwrap_or("~/bd.db".to_string());
        let path = PathBuf::from(&database_url);
        if path.exists() {
            return Err(CommandError::AlreadyFileExist);
        }
        File::create(path)?;
        let conn = establish_connection()?;
        create_bookmarks_table(&conn)?;
        Ok(CommandResult::Migrated(format!("Database file created file : {}", &database_url)))
    }
}
