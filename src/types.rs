use std::{env, io};

use anyhow::Result;
use derive_more::Display;
use thiserror::Error;

use crate::constants::SUCCESS_STRING;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("This key does not exist : {0}")]
    KeyNotFoundError(String),

    #[error("This key already exist : {0}")]
    KeyAlreadyExistError(String),

    #[error("This path already exist : {0}")]
    PathAlreadyExistError(String),

    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    VarError(#[from] env::VarError),

    #[error(transparent)]
    ConnectionErr(#[from] diesel::ConnectionError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Display, PartialEq)]
pub enum CommandResult {
    #[display(fmt = "")]
    DisplayNone,
    #[display(fmt = "{}", _0)]
    List(String),
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

pub type CliResult = Result<CommandResult, CommandError>;
