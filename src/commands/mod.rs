use structopt::{clap, StructOpt};

use crate::{
    constants::ERROR_STRING,
    types::{CommandError, CommandResult},
};

mod basic;
mod history;
mod util;

#[derive(Debug, StructOpt)]
#[structopt(name = "dirmarks")]
#[structopt(long_version(option_env ! ("LONG_VERSION").unwrap_or(env ! ("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub enum Opt {
    Init(util::init::Init),
    Migrate(util::migrate::Migrate),
    Completion(util::completion::Completion),
    Add(basic::add::Add),
    Jump(basic::jump::Jump),
    List(basic::list::List),
    Delete(basic::delete::Delete),
    Rename(basic::rename::Rename),
    History(history::History),
    Check(basic::check::Check),
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
            Opt::Check(check) => check.run(),
        };
        self.print_result(result);
    }

    pub fn print_result(&self, result: Result<CommandResult, CommandError>) {
        match result {
            Ok(command_result) => match command_result {
                CommandResult::DisplayNone => {},
                _ => println!("{}", command_result),
            },
            Err(err) => error!("{} : {}", *ERROR_STRING, err),
        }
    }
}
