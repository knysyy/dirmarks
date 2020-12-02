#[macro_use]
extern crate diesel;
#[macro_use]
extern crate prettytable;

pub use types::CliResult;

pub mod command;
pub mod database;
pub mod schema;
pub mod result;
pub mod error;
mod types;
