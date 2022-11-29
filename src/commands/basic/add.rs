use std::env;

use structopt::{clap, StructOpt};

use crate::{
    models::bookmark,
    types::{CliResult, CommandError, CommandResult},
    utils::database::establish_connection,
};
use anyhow::Context;

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
        let conn = &mut establish_connection()?;
        let current_dir =  env::current_dir()?;
        let path = match &self.path {
            Some(path) => path,
            None => current_dir.to_str().context("Failed to get current dir.")?
        };
        match bookmark::get_bookmark_by_key(conn, &self.key) {
            Ok(_) => Err(CommandError::KeyAlreadyExistError(self.key.clone())),
            Err(diesel::NotFound) => {
                match bookmark::get_bookmark_by_path(conn, &path) {
                    Ok(_) => Err(CommandError::PathAlreadyExistError(path.to_string())),
                    Err(diesel::NotFound) => {
                        bookmark::create_bookmark(conn, &self.key, &path, self.description.as_deref())?;
                        Ok(CommandResult::Added(self.key.to_string(), path.to_string()))
                    },
                    Err(err) => Err(CommandError::DieselError(err)),
                }
            },
            Err(err) => Err(CommandError::DieselError(err)),
        }
    }
}
