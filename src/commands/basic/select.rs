use std::io::Write;

use log::debug;
use structopt::{clap, StructOpt};

use crate::{
    commands::Command,
    fzf::Fzf,
    models::bookmark::{self, Order},
    types::{CliResult, CommandResult},
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "select", about = "select list")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Select {}

impl Command for Select {
    fn execute(&self) -> CliResult {
        debug!("{:?}", self);
        let conn = &mut establish_connection()?;
        let bookmarks = bookmark::get_bookmarks(conn, Order::Id, false)?;
        let mut fzf = Fzf::new()?;
        let handle = fzf.stdin();
        let max_length = bookmarks.iter().map(|bookmark| bookmark.key.len()).max().unwrap_or(0);
        for bookmark in &bookmarks {
            writeln!(handle, "{: <max_length$} : {}", bookmark.key, bookmark.path, max_length = max_length)?;
        }
        let selection = fzf.wait_select()?;
        let start_pos = max_length + 3;
        let end_pos = selection.len();
        Ok(CommandResult::Select(selection[start_pos..end_pos].to_string()))
    }
}
