use std::{env, fmt, fmt::Debug, io};

use diesel::{result::Error, ConnectionError};

#[derive(Debug)]
pub enum CommandError {
    NotFound,
    AlreadyExist,
    AlreadyFileExist,
    Database(diesel::result::Error),
    IoError(io::Error),
    VarError(env::VarError),
    ConnectionErr(diesel::ConnectionError),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::NotFound => write!(f, "This key does not exist"),
            CommandError::AlreadyExist => write!(f, "This key already exist"),
            CommandError::AlreadyFileExist => {
                write!(f, "Already database file exist")
            },
            CommandError::IoError(ref err) => fmt::Display::fmt(&err, f),
            CommandError::VarError(ref err) => fmt::Display::fmt(&err, f),
            CommandError::Database(Error::InvalidCString(ref err)) => {
                fmt::Display::fmt(&err, f)
            },
            CommandError::Database(Error::DatabaseError(_, ref err)) => {
                err.fmt(f)
            },
            CommandError::Database(Error::NotFound) => {
                write!(f, "Diesel Not Found")
            },
            CommandError::Database(Error::QueryBuilderError(ref err)) => {
                fmt::Display::fmt(&err, f)
            },
            CommandError::Database(Error::DeserializationError(ref err)) => {
                fmt::Display::fmt(&err, f)
            },
            CommandError::Database(Error::SerializationError(ref err)) => {
                fmt::Display::fmt(&err, f)
            },
            CommandError::Database(Error::RollbackTransaction) => {
                write!(f, "Diesel RollbackTransaction")
            },
            CommandError::Database(Error::AlreadyInTransaction) => {
                write!(f, "Diesel AlreadyInTransaction")
            },
            CommandError::ConnectionErr(ConnectionError::InvalidCString(
                ref err,
            )) => fmt::Display::fmt(&err, f),
            CommandError::ConnectionErr(ConnectionError::BadConnection(
                ref err,
            )) => fmt::Display::fmt(&err, f),
            CommandError::ConnectionErr(
                ConnectionError::CouldntSetupConfiguration(ref err),
            ) => fmt::Display::fmt(&err, f),
            CommandError::ConnectionErr(
                ConnectionError::InvalidConnectionUrl(ref err),
            ) => fmt::Display::fmt(&err, f),
            _ => write!(f, "Unknown Error"),
        }
    }
}

impl From<diesel::result::Error> for CommandError {
    fn from(err: diesel::result::Error) -> CommandError {
        CommandError::Database(err)
    }
}

impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> CommandError {
        CommandError::IoError(err)
    }
}

impl From<env::VarError> for CommandError {
    fn from(err: env::VarError) -> CommandError {
        CommandError::VarError(err)
    }
}

impl From<diesel::ConnectionError> for CommandError {
    fn from(err: diesel::ConnectionError) -> CommandError {
        CommandError::ConnectionErr(err)
    }
}
