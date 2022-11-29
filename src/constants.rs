use colored::{ColoredString, Colorize};
use once_cell::sync::Lazy;

pub static SUCCESS_STRING: Lazy<ColoredString> = Lazy::new(|| "Success".bold().cyan());
pub static ERROR_STRING: Lazy<ColoredString> = Lazy::new(|| "Error".bold().red());
