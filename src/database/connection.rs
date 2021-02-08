use std::env;

use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenv::dotenv;

use crate::error::CommandError;

pub fn establish_connection() -> Result<SqliteConnection, CommandError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)?)
}
