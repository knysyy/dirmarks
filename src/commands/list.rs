use std::borrow::Borrow;

use diesel::SqliteConnection;
use prettytable::{format, Table};
use structopt::{clap, clap::ArgGroup, StructOpt};

use crate::{
    models::{
        bookmark::{self, Bookmark},
        result::CommandResult,
    },
    types::CliResult,
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "list", about = "list directory")]
#[structopt(group = ArgGroup::with_name("option").required(false))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct List {
    #[structopt(short, long, group = "option")]
    raw: bool,
}

impl List {
    pub fn run(&self) -> CliResult {
        let conn = establish_connection()?;
        if self.raw {
            return Ok(self.show_bookmark_raw(&conn)?);
        }
        return Ok(self.show_bookmark(&conn)?);
    }

    fn show_bookmark_raw(&self, conn: &SqliteConnection) -> CliResult {
        let results: Vec<Bookmark> = bookmark::get_bookmarks(&conn)?;
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
        print!("{}", keys);
        Ok(CommandResult::DisplayNone)
    }

    fn show_bookmark(&self, conn: &SqliteConnection) -> CliResult {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.add_row(row![bFc => "id", "key", "path", "description"]);
        let results: Vec<Bookmark> = bookmark::get_bookmarks(&conn)?;
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
