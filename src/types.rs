use crate::error::CommandError;
use crate::result::CommandResult;

pub type CliResult = Result<CommandResult, CommandError>;
