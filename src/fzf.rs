use std::process::{Child, ChildStdin, Command, Stdio};

use anyhow::{bail, Context, Result};

pub struct Fzf {
    child: Child,
}

impl Fzf {
    pub fn new() -> Result<Self> {
        let mut command = Command::new("fzf");
        command.stdin(Stdio::piped()).stdout(Stdio::piped());

        Ok(Fzf {
            child: command.spawn().context("could not launch fzf")?,
        })
    }

    pub fn stdin(&mut self) -> &mut ChildStdin {
        self.child.stdin.as_mut().unwrap()
    }

    pub fn wait_select(self) -> Result<String> {
        let output = self
            .child
            .wait_with_output()
            .context("wait failed on fzf")?;

        match output.status.code() {
            Some(0) => String::from_utf8(output.stdout).context("invalid unicode in fzf output"),
            Some(1) => bail!("no match found"),
            Some(2) => bail!("fzf returned an error"),
            Some(128..=254) | None => bail!("fzf was terminated"),
            _ => bail!("fzf returned an unknown error"),
        }
    }
}
