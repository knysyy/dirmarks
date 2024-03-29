use std::path::Path;
use log::debug;

use structopt::{clap, StructOpt};

use crate::{
    models::{bookmark, bookmark::Order},
    types::{CliResult, CommandResult},
    utils::database::establish_connection,
};
use crate::commands::Command;

#[derive(Debug, StructOpt)]
#[structopt(name = "check", about = "check directory")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Check {}

impl Command for Check {
    fn execute(&self) -> CliResult {
        debug!("{:?}", self);
        let conn = &mut establish_connection()?;
        let results = bookmark::get_bookmarks(conn, Order::Id, false)?;
        let mut exist = false;
        for bookmark in results {
            if !Path::new(&bookmark.path).exists() {
                exist = true;
                println!(
                    "Directory {} is Not Found key {}",
                    bookmark.path, bookmark.key
                );
            }
        }
        if !exist {
            println!("No Directory without path");
        }
        Ok(CommandResult::DisplayNone)
    }
}
