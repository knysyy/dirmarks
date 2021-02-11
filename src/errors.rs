use std::{env, fmt::Debug, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("This key does not exist : {0}")]
    KeyNotFoundError(String),

    #[error("This key already exist : {0}")]
    KeyAlreadyExistError(String),

    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    VarError(#[from] env::VarError),

    #[error(transparent)]
    ConnectionErr(#[from] diesel::ConnectionError),
}
