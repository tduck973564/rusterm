// Copyright 2021 Thomas Duckworth <tduck973564@gmail.com>.
// This file is part of the `brc` project, licenced under the GNU GPL v3.0, which can be read here: https://www.gnu.org/licenses/gpl-3.0.en.html

//! Module containing lexing and tokenising functionality for turning scanned user input into something that can be operated on by the commands.

use crate::error;
use std::convert::TryFrom;

/// The enum containing the different variants of possible arguments. Can be converted into `String`, `i32`, or `f64`, depending on the variant.
#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    /// The string token for arguments, can be converted into `String`.
    String(String),
    /// The integer token, can be converted into `i32`.
    Integer(i32),
    /// The float token, can be converted into `f64`.
    Float(f64),
}

/// For popping the first argument from the `Arguments` Vec, and returning it, or returning an error from `brc::error::Error`. To be used with `try_into()` to convert it to the expected type.
pub trait PopArgument {
    /// Pops the first argument from an `Arguments` Vec, and returns it. If it is popped when the vec is empty, `Error::BadArgumentsLen` will be returned.
    fn pop_arg(&mut self) -> Result<Argument, error::Error>;
}

impl PopArgument for Arguments {
    fn pop_arg(&mut self) -> Result<Argument, error::Error> {
        let value: Result<Argument, error::Error> = match self.get(0) {
            None => return Err(error::Error::BadArgumentsLen),
            Some(x) => Ok(x.clone()),
        };
        self.remove(0);
        value
    }
}

/// Arguments type to represent a Vec with `Argument`s in it.
pub type Arguments = Vec<Argument>;

impl From<String> for Argument {
    fn from(input: String) -> Self {
        Argument::String(input)
    }
}

impl From<i32> for Argument {
    fn from(input: i32) -> Self {
        Argument::Integer(input)
    }
}

impl TryFrom<Argument> for String {
    type Error = error::Error;
    fn try_from(input: Argument) -> Result<Self, Self::Error> {
        match input {
            Argument::String(x) => Ok(x),
            _ => Err(Self::Error::InvalidArgument),
        }
    }
}

impl TryFrom<Argument> for i32 {
    type Error = error::Error;
    fn try_from(input: Argument) -> Result<Self, Self::Error> {
        match input {
            Argument::Integer(x) => Ok(x),
            _ => Err(error::Error::InvalidArgument),
        }
    }
}

impl TryFrom<Argument> for f64 {
    type Error = error::Error;
    fn try_from(input: Argument) -> Result<Self, Self::Error> {
        match input {
            Argument::Float(x) => Ok(x),
            _ => Err(Self::Error::InvalidArgument),
        }
    }
}

/// Lexes and tokenises the scanned user input into variants of `Argument`.
/// # Examples
/// `33` will become `Argument::Integer(33)`
/// `"hello, world"` will become `Argument::String("hello, world")`
/// `4.2` will become `Argument::Float(4.2)`
pub fn lex(input: Vec<String>) -> Arguments {
    let mut output: Vec<Argument> = Vec::new();

    for argument in input {
        match argument.parse::<i32>() {
            Ok(argument_as_integer) => output.push(Argument::Integer(argument_as_integer)),
            Err(_) => output.push({
                match argument.parse::<f64>() {
                    Ok(argument_as_float) => Argument::Float(argument_as_float),
                    Err(_) => Argument::String(argument),
                }
            }),
        }
    }
    output
}
