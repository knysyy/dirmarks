use std::env;

use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::errors::CommandError;

pub fn establish_connection() -> Result<SqliteConnection, CommandError> {
    let database_url = env::var("DM_DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)?)
}
