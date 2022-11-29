use anyhow::Result;
use diesel::{prelude::*, sql_query, Insertable, Queryable, SqliteConnection};

use crate::schema::bookmarks;

#[derive(Debug, Queryable)]
pub struct Bookmark {
    pub id: i32,
    pub key: String,
    pub path: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = bookmarks)]
pub struct NewBookmark<'a> {
    pub key: &'a str,
    pub path: &'a str,
    pub description: Option<&'a str>,
}

pub enum Order {
    Id,
    Key,
    Path,
}

pub fn create_bookmarks_table(conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
    // TODO load sql or diesel_migration
    sql_query("CREATE TABLE IF NOT EXISTS bookmarks(id INTEGER PRIMARY KEY, key VARCHAR(10) NOT NULL, path VARCHAR(255) NOT NULL, description TEXT)")
        .execute(conn)?;
    Ok(())
}

pub fn get_bookmarks(
    conn: &mut SqliteConnection,
    order: Order,
    desc: bool,
) -> Result<Vec<Bookmark>, diesel::result::Error> {
    use crate::schema::bookmarks::*;
    let mut query = bookmarks::table.into_boxed();
    if desc {
        query = match order {
            Order::Id => query.order(id.desc()),
            Order::Key => query.order(key.desc()),
            Order::Path => query.order(path.desc()),
        };
    } else {
        query = match order {
            Order::Id => query.order(id.asc()),
            Order::Key => query.order(key.asc()),
            Order::Path => query.order(path.asc()),
        };
    }
    query.load::<Bookmark>(conn)
}

pub fn get_bookmark_by_key(
    conn: &mut SqliteConnection,
    input_key: &str,
) -> Result<Bookmark, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    bookmarks.filter(key.eq(input_key)).first::<Bookmark>(conn)
}

pub fn get_bookmark_by_path(
    conn: &mut SqliteConnection,
    input_path: &str,
) -> Result<Bookmark, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    bookmarks
        .filter(path.eq(input_path))
        .first::<Bookmark>(conn)
}

pub fn create_bookmark<'a>(
    conn: &mut SqliteConnection,
    key: &'a str,
    path: &'a str,
    description: Option<&'a str>,
) -> Result<usize, diesel::result::Error> {
    let new_bookmark = NewBookmark {
        key,
        path,
        description,
    };

    diesel::insert_into(bookmarks::table)
        .values(&new_bookmark)
        .execute(conn)
}

pub fn delete_bookmark(
    conn: &mut SqliteConnection,
    input_key: &str,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    diesel::delete(bookmarks.filter(key.eq(input_key))).execute(conn)
}

pub fn rename_bookmark(
    conn: &mut SqliteConnection,
    old_key: &str,
    new_key: &str,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    diesel::update(bookmarks.filter(key.eq(old_key)))
        .set(key.eq(new_key))
        .execute(conn)
}
