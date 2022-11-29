use anyhow::Context;
use diesel::SqliteConnection;
use log::debug;
use prettytable::{format, row, Table};
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
        let conn = &mut establish_connection()?;
        if self.raw {
            return Ok(self.show_bookmark_raw(conn)?);
        }
        return Ok(self.show_bookmark(conn)?);
    }

    fn get_bookmarks(&self, conn: &mut SqliteConnection) -> anyhow::Result<Vec<Bookmark>> {
        if self.key {
            bookmark::get_bookmarks(conn, Order::Key, self.desc)
        } else if self.path {
            bookmark::get_bookmarks(conn, Order::Path, self.desc)
        } else if self.id {
            bookmark::get_bookmarks(conn, Order::Id, self.desc)
        } else {
            Ok(Vec::new())
        }
        .context("ブックマークの取得に失敗しました。")
    }

    fn show_bookmark_raw(&self, conn: &mut SqliteConnection) -> CliResult {
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
                        .unwrap_or(&String::from("no description"))
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        print!("{}", keys);
        Ok(CommandResult::DisplayNone)
    }

    fn show_bookmark(&self, conn: &mut SqliteConnection) -> CliResult {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.add_row(row![bFc => "id", "key", "path", "description"]);
        let results = self.get_bookmarks(conn)?;
        println!("Displaying {} directories", results.len());
        for bookmark in results {
            match bookmark.description {
                Some(value) => {
                    table.add_row(row![bookmark.id, bookmark.key, bookmark.path, value]);
                },
                None => {
                    table.add_row(row![bookmark.id, bookmark.key, bookmark.path, Fr -> "None"]);
                },
            }
        }
        table.printstd();
        Ok(CommandResult::DisplayNone)
    }
}
