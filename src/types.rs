use crate::{error::CommandError, result::CommandResult};

pub type CliResult = Result<CommandResult, CommandError>;
