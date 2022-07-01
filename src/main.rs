#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prettytable;

use dotenv::dotenv;
use structopt::StructOpt;

use crate::commands::Opt;

pub mod commands;
pub mod constants;
pub mod fzf;
pub mod models;
pub mod schema;
pub mod types;
pub mod utils;

fn main() {
    dotenv().ok();
    env_logger::init();
    let opt: Opt = Opt::from_args();
    debug!("{:?}", opt);
    opt.run();
}
