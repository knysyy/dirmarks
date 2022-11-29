use std::env;

use dotenv::dotenv;
use log::debug;
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
    let result = dotenv();
    if result.is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let opt: Opt = Opt::from_args();
    debug!("{:?}", opt);
    opt.run();
}
