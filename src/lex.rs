#[derive(Debug, PartialEq)]
pub enum Arguments {
    String(String),
    Number(i32),
}

impl From<String> for Arguments {
    fn from(input: String) -> Self {
        Arguments::String(input)
    }
}

impl From<i32> for Arguments {
    fn from(input: i32) -> Self {
        Arguments::Number(input)
    }
}

pub fn lex(input: Vec<String>) -> Vec<Arguments> {
    let mut output: Vec<Arguments> = Vec::new();
    for argument in input {
        match argument.parse::<i32>() {
            Ok(argument_as_number) => output.push(Arguments::Number(argument_as_number)),
            Err(_) => output.push(Arguments::String(argument))
        }
    }
    output
}