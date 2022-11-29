mod basic;
mod history;
mod util;

use enum_dispatch::enum_dispatch;
use structopt::{clap, StructOpt};

use crate::{
    constants::ERROR_STRING,
    types::{CliResult, CommandResult},
};

#[enum_dispatch(Opt)]
pub trait Command {
    fn execute(&self) -> CliResult;
}

#[enum_dispatch]
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
}

impl Opt {
    pub fn run(&self) {
        let result: CliResult = self.execute();
        match result {
            Ok(command_result) => match command_result {
                CommandResult::DisplayNone => {},
                _ => println!("{}", command_result),
            },
            Err(err) => println!("{} : {}", *ERROR_STRING, err),
        }
    }
}
