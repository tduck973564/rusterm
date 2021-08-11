use crate::error;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    String(String),
    Number(i32),
}

impl From<String> for Argument {
    fn from(input: String) -> Self {
        Argument::String(input)
    }
}

impl From<i32> for Argument {
    fn from(input: i32) -> Self {
        Argument::Number(input)
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
            Argument::Number(x) => Ok(x),
            _ => Err(Self::Error::InvalidArgument),
        }
    }
}

pub fn lex(input: Vec<String>) -> Vec<Argument> {
    let mut output: Vec<Argument> = Vec::new();
    for argument in input {
        match argument.parse::<i32>() {
            Ok(argument_as_number) => output.push(Argument::Number(argument_as_number)),
            Err(_) => output.push(Argument::String(argument)),
        }
    }
    output
}
