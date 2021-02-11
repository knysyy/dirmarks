use anyhow::Result;
use diesel::{prelude::*, sql_query, Insertable, Queryable, SqliteConnection};

use crate::schema::histories;

#[derive(Debug, Queryable)]
pub struct History {
    pub id: i32,
    pub path: String,
}

#[derive(Insertable)]
#[table_name = "histories"]
pub struct NewHistory<'a> {
    pub path: &'a str,
}

pub fn create_histories_table(conn: &SqliteConnection) -> Result<(), diesel::result::Error> {
    sql_query(
        "CREATE TABLE IF NOT EXISTS histories(id INTEGER PRIMARY KEY, path VARCHAR(255) NOT NULL)",
    )
    .execute(conn)?;
    Ok(())
}

pub fn create_histories(
    conn: &SqliteConnection,
    path: &str,
) -> Result<usize, diesel::result::Error> {
    let new_history = NewHistory { path };

    diesel::insert_into(histories::table)
        .values(&new_history)
        .execute(conn)
}
