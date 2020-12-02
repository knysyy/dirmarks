use colored::*;
use structopt::{clap, StructOpt};

use crate::error::CommandError;
use crate::result::CommandResult;

#[derive(Debug, StructOpt)]
#[structopt(name = "bookmark-directory")]
#[structopt(long_version(option_env ! ("LONG_VERSION").unwrap_or(env ! ("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub enum Opt {
    Init(super::Init),
    Add(super::Add),
    Jump(super::Jump),
    List(super::List),
    Delete(super::Delete),
    Rename(super::Rename),
    Migrate(super::Migrate),
}

impl Opt {
    pub fn run(&self) {
        let result = match self {
            Opt::Init(init) => init.run(),
            Opt::Add(add) => add.run(),
            Opt::Jump(jump) => jump.run(),
            Opt::List(list) => list.run(),
            Opt::Delete(delete) => delete.run(),
            Opt::Rename(rename) => rename.run(),
            Opt::Migrate(migrate) => migrate.run(),
        };
        self.print_result(result);
    }

    pub fn print_result(&self, result: Result<CommandResult, CommandError>) {
        let success = "Success".bold().cyan();
        let error = "Error".bold().red();
        match result {
            Ok(command_result) => {
                match command_result {
                    CommandResult::Added(key, path) => {
                        println!("{} : Added {} -> {}", success, key, path);
                    }
                    CommandResult::Deleted(key, path) => {
                        println!("{} : Deleted {} -> {}", success, key, path);
                    }
                    CommandResult::Renamed(old_key, new_key) => {
                        println!("{} : Renamed {} -> {}", success, old_key, new_key);
                    }
                    CommandResult::Jump(path) => {
                        println!("jump : {}", path);
                    }
                    CommandResult::Migrated(path) => {
                        println!("{} : Migrated {}", success, path);
                    }
                    _ => {}
                }
            }
            Err(err) => println!("{} : {}", error, err)
        }
    }
}
