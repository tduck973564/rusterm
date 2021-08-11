use colored::Colorize as Colourise;
use std::error::Error as ErrorTrait;
use std::fmt; // Smh use correct english

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
            Error::NoSuchCommand => write!(
                f,
                "{}{} You typed in the name of a command that does not exist.",
                "NoSuchCommand".red().bold(),
                ":".bold(),
            ),
            Error::InvalidArgument => write!(
                f,
                "{}{} You passed an argument that is of incorrect type.",
                "InvalidArgument".red().bold(),
                ":".bold(),
            ),
            Error::BadArgumentsLen => write!(
                f,
                "{}{} You passed too many or too little arguments to the command.",
                "BadArgumentsLen".red().bold(),
                ":".bold(),
            ),
        }
    }
}

impl ErrorTrait for Error {}
