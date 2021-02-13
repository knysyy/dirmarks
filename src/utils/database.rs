use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::{types::CommandError, utils::config::CONFIG};

pub fn establish_connection() -> Result<SqliteConnection, CommandError> {
    Ok(SqliteConnection::establish(&CONFIG.database_url)?)
}
