use anyhow::Result;

use crate::{errors::CommandError, models::result::CommandResult};

pub type CliResult = Result<CommandResult, CommandError>;
