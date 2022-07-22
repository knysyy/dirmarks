use std::borrow::Borrow;

use anyhow::Context;
use diesel::SqliteConnection;
use prettytable::{format, Table};
use structopt::{clap, StructOpt};

use crate::{
    models::bookmark::{self, Bookmark, Order},
    types::{CliResult, CommandResult},
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "list", about = "list directory")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct List {
    #[structopt(short, long)]
    raw: bool,

    #[structopt(short, long, conflicts_with_all(&["key", "path"]))]
    id: bool,

    #[structopt(short, long, conflicts_with_all(&["id", "path"]))]
    key: bool,

    #[structopt(short, long, conflicts_with_all(&["id", "key"]))]
    path: bool,

    #[structopt(short, long)]
    desc: bool,
}

impl List {
    pub fn run(&self) -> CliResult {
        debug!("{:?}", self);
        let conn = establish_connection()?;
        if self.raw {
            self.show_bookmark_raw(&conn)
        } else {
            self.show_bookmark(&conn)
        }
    }

    fn get_bookmarks(&self, conn: &SqliteConnection) -> anyhow::Result<Vec<Bookmark>> {
        if self.key {
            bookmark::get_bookmarks(conn, Order::Key, self.desc)
        } else if self.path {
            bookmark::get_bookmarks(conn, Order::Path, self.desc)
        } else if self.id {
            bookmark::get_bookmarks(conn, Order::Id, self.desc)
        } else {
            bookmark::get_bookmarks(conn, Order::Id, self.desc)
        }
        .context("Failed to get bookmarks")
    }

    fn show_bookmark_raw(&self, conn: &SqliteConnection) -> CliResult {
        let results = self.get_bookmarks(conn)?;
        let keys = results
            .iter()
            .map(|bookmark| {
                format!(
                    "'{}[{}]'",
                    bookmark.key,
                    bookmark
                        .description
                        .as_ref()
                        .unwrap_or("no description".to_string().borrow())
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        Ok(CommandResult::List(keys))
    }

    fn show_bookmark(&self, conn: &SqliteConnection) -> CliResult {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row![bFc => "id", "key", "path", "description"]);
        let results = self.get_bookmarks(conn)?;
        for bookmark in &results {
            match &bookmark.description {
                Some(value) => {
                    table.add_row(row![bookmark.id, bookmark.key, bookmark.path, value]);
                }
                None => {
                    table.add_row(row![bookmark.id, bookmark.key, bookmark.path, Fr -> "None"]);
                }
            }
        }
        // TODO 色が付かなくなったので修正。
        Ok(CommandResult::List(format!("Displaying {} directories\n{}", results.len(), table.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use dotenv;

    use crate::models::bookmark;
    use crate::utils::config::CONFIG;
    use crate::types::CommandResult;

    use super::*;

    fn setup() {
        dotenv::from_path(".env.test").ok();
        fs::remove_file(&CONFIG.database_url).unwrap();
        let conn = establish_connection().unwrap();
        bookmark::create_bookmarks_table(&conn).unwrap();
        bookmark::create_bookmark(&conn, "key1", "/example", Option::Some("description")).unwrap();
        bookmark::create_bookmark(&conn, "key2", "/example2", Option::Some("description2")).unwrap();
    }

    #[test]
    fn show_bookmark_raw_no_option() {
        setup();
        let conn = establish_connection().unwrap();
        let list = List { raw: true, id: false, key: false, path: false, desc: false };
        let expected = CommandResult::List("'key1[description]' 'key2[description2]'".to_string());
        let result = list.show_bookmark_raw(&conn).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn show_bookmark_no_option() {
        setup();
        let conn = establish_connection().unwrap();
        let list = List { raw: false, id: false, key: false, path: false, desc: false };
        let expected = "Displaying 2 directories\n".to_string()
            + " id | key  | path      | description \n"
            + "----+------+-----------+--------------\n"
            + " 1  | key1 | /example  | description \n"
            + " 2  | key2 | /example2 | description2 \n";
        let result = list.show_bookmark(&conn).unwrap();
        assert_eq!(CommandResult::List(expected), result);
    }
}
