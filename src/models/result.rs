use derive_more::Display;

use crate::constants::SUCCESS_STRING;

#[derive(Display)]
pub enum CommandResult {
    #[display(fmt = "")]
    DisplayNone,
    #[display(fmt = "{} : Added {} -> {}", "*SUCCESS_STRING", _0, _1)]
    Added(String, String),
    #[display(fmt = "{} : Deleted {} -> {}", "*SUCCESS_STRING", _0, _1)]
    Deleted(String, String),
    #[display(fmt = "{} : Renamed {} -> {}", "*SUCCESS_STRING", _0, _1)]
    Renamed(String, String),
    #[display(fmt = "jump : {}", _0)]
    Jump(String),
    #[display(fmt = "{} : Migrated {}", "*SUCCESS_STRING", _0)]
    Migrated(String),
}
