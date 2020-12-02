use diesel::{Queryable, Insertable};
use crate::schema::*;

#[derive(Debug, Queryable)]
pub struct Bookmark {
    pub id: i32,
    pub key: String,
    pub path: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "bookmarks"]
pub struct NewBookmark<'a> {
    pub key: &'a str,
    pub path: &'a str,
    pub description: Option<&'a str>,
}