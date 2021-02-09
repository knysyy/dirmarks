use colored::{ColoredString, Colorize};
use once_cell::sync::Lazy;

pub const SUCCESS_STRING: Lazy<ColoredString> = Lazy::new(|| "Success".bold().cyan());
pub const ERROR_STRING: Lazy<ColoredString> = Lazy::new(|| "Error".bold().red());
