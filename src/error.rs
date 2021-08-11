// Copyright 2021 Thomas Duckworth <tduck973564@gmail.com>.
// This file is part of the `brc` project, licenced under the GNU GPL v3.0, which can be read here: https://www.gnu.org/licenses/gpl-3.0.en.html

//! Module containing the error enum, and an implementation of `std::error::Error` over it.

use colored::Colorize as Colourise;
use std::error::Error as ErrorTrait;
use std::fmt; // Smh use correct english

/// The enum containing all of the different error variants. Implements `std::error::Error`.
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
