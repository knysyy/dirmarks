use diesel::{prelude::*, sql_query, SqliteConnection};

use super::model::{Bookmark, NewBookmark};

pub fn create_bookmarks_table(
    conn: &SqliteConnection,
) -> Result<(), diesel::result::Error> {
    sql_query("CREATE TABLE bookmarks(id INTEGER PRIMARY KEY, key VARCHAR(10) NOT NULL, path VARCHAR(255) NOT NULL, description TEXT)")
        .execute(conn)?;
    Ok(())
}

pub fn get_bookmarks(
    conn: &SqliteConnection,
) -> Result<Vec<Bookmark>, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::bookmarks;
    bookmarks.load::<Bookmark>(conn)
}

pub fn get_bookmark(
    conn: &SqliteConnection,
    input_key: &str,
) -> Result<Bookmark, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    bookmarks.filter(key.eq(input_key)).first::<Bookmark>(conn)
}

pub fn create_bookmark<'a>(
    conn: &SqliteConnection,
    key: &'a str,
    path: &'a str,
    description: Option<&'a str>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::bookmarks;

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
    conn: &SqliteConnection,
    input_key: &str,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    diesel::delete(bookmarks.filter(key.eq(input_key))).execute(conn)
}

pub fn rename_bookmark(
    conn: &SqliteConnection,
    old_key: &str,
    new_key: &str,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    diesel::update(bookmarks.filter(key.eq(old_key)))
        .set(key.eq(new_key))
        .execute(conn)
}
