mod add;
mod completion;
mod delete;
mod history;
mod init;
mod jump;
mod list;
mod migrate;
mod rename;

use structopt::{clap, StructOpt};

use crate::{
    constants::ERROR_STRING,
    types::{CommandError, CommandResult},
};

#[derive(Debug, StructOpt)]
#[structopt(name = "dirmarks")]
#[structopt(long_version(option_env ! ("LONG_VERSION").unwrap_or(env ! ("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub enum Opt {
    Init(init::Init),
    Add(add::Add),
    Jump(jump::Jump),
    List(list::List),
    Delete(delete::Delete),
    Rename(rename::Rename),
    Migrate(migrate::Migrate),
    History(history::History),
    Completion(completion::Completion),
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
            Opt::History(history) => history.run(),
            Opt::Completion(completion) => completion.run(),
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
