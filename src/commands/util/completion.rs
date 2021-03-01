use std::io;

use structopt::{
    clap::{self, Shell},
    StructOpt,
};

use crate::{
    commands::Opt,
    types::{CliResult, CommandResult},
};

#[derive(Debug, StructOpt)]
#[structopt(name = "completion", about = "print completion")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Completion {
    #[structopt(subcommand)]
    shell: ShellOpt,
}

#[derive(Debug, StructOpt)]
enum ShellOpt {
    #[structopt(name = "zsh", about = "print zsh completion")]
    Zsh,
    #[structopt(name = "bash", about = "print bash completion")]
    Bash,
}

impl Completion {
    pub fn run(&self) -> CliResult {
        debug!("{:?}", self);
        match self.shell {
            ShellOpt::Zsh => {
                Self::completion(Shell::Zsh);
            },
            ShellOpt::Bash => {
                Self::completion(Shell::Bash);
            },
        }
        Ok(CommandResult::DisplayNone)
    }

    fn completion(shell: Shell) {
        Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut io::stdout())
    }
}
