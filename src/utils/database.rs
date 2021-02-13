use anyhow::{Context, Result};
use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::utils::config::CONFIG;

pub fn establish_connection() -> Result<SqliteConnection> {
    Ok(SqliteConnection::establish(&CONFIG.database_url)
        .context("Failed to connect to the database.")?)
}
