use std::error::Error as ErrorTrait;
use std::fmt;
use colored::Colorize as Colourise; // Smh use correct english

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    NoSuchCommand,
    InvalidArgument,
    BadArgumentsLen,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoSuchCommand => write!(f, "{}{} {}", "NoSuchCommand".red().bold(), ":".bold(), "You typed in the name of a command that does not exist."),
            Error::InvalidArgument => write!(f, "{}{} {}", "InvalidArgument".red().bold(), ":".bold(), "You passed an argument that is of incorrect type."),
            Error::BadArgumentsLen => write!(f, "{}{} {}", "BadArgumentsLen".red().bold(), ":".bold(), "You passed too many or too little arguments to the command."),
        }
    }
}

impl ErrorTrait for Error {}
