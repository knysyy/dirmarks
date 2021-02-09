use std::{io, io::Write};

use structopt::{clap, StructOpt};

use crate::{models::result::CommandResult, types::CliResult};

#[derive(Debug, StructOpt)]
#[structopt(name = "init", about = "bookmark directory init commands")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Init {}

impl Init {
    pub fn run(&self) -> CliResult {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", ZSH).unwrap();
        writeln!(handle, "{}", ZSH_ALIAS).unwrap();
        writeln!(handle, "{}", ZSH_AUTOCOMPLETE).unwrap();
        Ok(CommandResult::DisplayNone)
    }
}

const ZSH: &str = r#"
_bj_cd() {
    cd "$@" || return "$?"
}

bj() {
    if [ "$#" -eq 0 ]; then
        _bj_cd ~ || return "$?"
    elif [ "$#" -eq 1 ] && [ "$1" = '-' ]; then
        _bj_cd ~- || return "$?"
    else
        result="$(bookmark-directory jump $@)" || return "$?"
        case "$result" in
            "jump : "*)
                _bj_cd "${result:7}" || return "$?"
                ;;
            *)
                if [ -n "$result" ]; then
                    echo "$result"
                fi
                ;;
        esac
    fi
}
"#;

const ZSH_ALIAS: &str = r#"
alias ba='bookmark-directory add'
alias bl='bookmark-directory list'
alias bd='bookmark-directory delete'
alias br='bookmark-directory rename'
"#;

const ZSH_AUTOCOMPLETE: &str = r#"
compdef _bj bj
_bj() {
  local results
  local values
  results=$(bookmark-directory list -r)
  if [[ -z "$results" ]]; then
    return 0;
  fi
  str=$(echo $results)
  eval "values=($str)"
  _values 'keys' $values
  return 1;
}
"#;
