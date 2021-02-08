#[macro_use]
extern crate diesel;
#[macro_use]
extern crate prettytable;

use structopt::StructOpt;

use crate::command::Opt;

pub mod command;
pub mod database;
pub mod error;
pub mod result;
pub mod schema;
pub mod types;

fn main() {
    let opt: Opt = Opt::from_args();
    opt.run();
}
