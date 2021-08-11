use crate::error;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    String(String),
    Integer(i32),
    Float(f64),
}

pub trait PopArgument {
    fn pop_arg(&mut self) -> Result<Argument, error::Error>;
}

impl PopArgument for Vec<Argument> {
    fn pop_arg(&mut self) -> Result<Argument, error::Error> {
        let value: Result<Argument, error::Error> = match self.get(0) {
            None => return Err(error::Error::BadArgumentsLen),
            Some(x) => Ok(x.clone()),
        };
        self.remove(0);
        value
    }
}

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

