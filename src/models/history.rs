use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use diesel::{prelude::*, sql_query, Insertable, Queryable, SqliteConnection};

use crate::schema::histories;

#[derive(Debug, Queryable)]
pub struct History {
    pub id: i32,
    pub path: String,
    pub count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "histories"]
pub struct NewHistory<'a> {
    pub path: &'a str,
    pub count: i32,
}

pub fn create_histories_table(conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
    // TODO load sql or diesel_migration
    sql_query(
        "CREATE TABLE histories (
        id INTEGER PRIMARY KEY,
        path VARCHAR(255) NOT NULL UNIQUE,
        count INTEGER DEFAULT 0 NOT NULL,
        created_at TEXT DEFAULT (DATETIME('now', 'localtime')) NOT NULL,
        updated_at TEXT DEFAULT (DATETIME('now', 'localtime')) NOT NULL
    );",
    )
    .execute(conn)?;
    Ok(())
}

pub fn create_histories(
    conn: &mut SqliteConnection,
    path: &str,
) -> Result<usize, diesel::result::Error> {
    let new_history = NewHistory { path, count: 1 };
    diesel::insert_into(histories::table)
        .values(&new_history)
        .execute(conn)
}

pub fn get_history(
    conn: &mut SqliteConnection,
    input_path: &str,
) -> Result<History, diesel::result::Error> {
    use crate::schema::histories::dsl::*;
    histories.filter(path.eq(input_path)).first::<History>(conn)
}

pub fn get_histories(conn: &mut SqliteConnection) -> Result<Vec<History>, diesel::result::Error> {
    use crate::schema::histories::dsl::*;
    histories.load::<History>(conn)
}

pub fn update_count(
    conn: &mut SqliteConnection,
    input_path: &str,
    input_count: i32,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::histories::dsl::*;
    diesel::update(histories.filter(path.eq(input_path)))
        .set((
            count.eq(input_count),
            updated_at.eq(Local::now().naive_local()),
        ))
        .execute(conn)
}
