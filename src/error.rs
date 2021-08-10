use std::error::Error as ErrorTrait;
use std::fmt;

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
            Error::NoSuchCommand => write!(f, "NoSuchCommand: You typed in the name of a command that does not exist."),
            Error::InvalidArgument => write!(f, "InvalidArgument: You passed an argument that is of incorrect type."),
            Error::BadArgumentsLen => write!(f, "BadArgumentsLen: You passed too many or too little arguments to the command."),
        }
    }
}

impl ErrorTrait for Error {}
