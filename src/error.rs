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
    /// The error variant returned when a command is looked up in the command table, but `None` is returned. Occurs when typing a command that has not been put into the command table.
    NoSuchCommand,
    /// The error variant for when an argument is not of the correct type.
    InvalidArgument,
    /// The error variant for when not enough arguments were passed to the command. Occurs when popping an empty `Arguments` Vec.
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
                "{}{} You passed too little arguments to the command.",
                "BadArgumentsLen".red().bold(),
                ":".bold(),
            ),
        }
    }
}

impl ErrorTrait for Error {}
