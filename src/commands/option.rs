use structopt::{clap, StructOpt};

use crate::{constants::ERROR_STRING, errors::CommandError, models::result::CommandResult};

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
        match result {
            Ok(command_result) => match command_result {
                CommandResult::DisplayNone => {},
                _ => println!("{}", command_result),
            },
            Err(err) => println!("{} : {}", *ERROR_STRING, err),
        }
    }
}
