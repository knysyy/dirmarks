use std::{io, io::Write};

use log::debug;
use structopt::{clap, StructOpt};

use crate::{
    commands::Command,
    types::{CliResult, CommandResult},
};

#[derive(Debug, StructOpt)]
#[structopt(name = "init", about = "bookmark directory init commands")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Init {}

impl Command for Init {
    fn execute(&self) -> CliResult {
        debug!("{:?}", self);
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", ZSH).unwrap();
        writeln!(handle, "{}", ZSH_ALIAS).unwrap();
        writeln!(handle, "{}", ZSH_AUTOCOMPLETE).unwrap();
        Ok(CommandResult::DisplayNone)
    }
}

const ZSH: &str = r#"
_dirmarks_cd() {
    cd "$@" || return "$?"
}

_dirmarks_jump() {
  if [ -p /dev/stdin ]; then
    result="$(cat /dev/stdin)"
  else
    result=$@
  fi
  case "$result" in
    "jump : "*)
        _dirmarks_cd "${result:7}" || return "$?"
        ;;
    *)
        if [ -n "$result" ]; then
            echo "$result"
        fi
        ;;
  esac
}

bj() {
    if [ "$#" -eq 0 ]; then
        _dirmarks_cd ~ || return "$?"
    elif [ "$#" -eq 1 ] && [ "$1" = '-' ]; then
        _dirmarks_cd ~- || return "$?"
    else
        result="$(dirmarks jump $@)" || return "$?"
        _dirmarks_jump $result
    fi
}

bs() {
    dirmarks select $@ | _dirmarks_jump
}

bh() {
    dirmarks history jump $@ | _dirmarks_jump
}
"#;

const ZSH_ALIAS: &str = r#"
alias ba='dirmarks add'
alias bl='dirmarks list'
alias bd='dirmarks delete'
alias br='dirmarks rename'
alias bc='dirmarks check'
"#;

const ZSH_AUTOCOMPLETE: &str = r#"
compdef _bj bj
_bj() {
  local results
  local values
  results=$(dirmarks list -r)
  if [[ -z "$results" ]]; then
    return 0;
  fi
  str=$(echo $results)
  eval "values=($str)"
  _values 'keys' $values
  return 1;
}
"#;
