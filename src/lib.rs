use std::error::Error;
use std::collections::HashMap;
mod io;


pub struct Console {
    command_table: HashMap<String, fn()>,
    prompt: String,
}

pub enum Arguments {
    String(String),
    Number(i32),
}

fn nothing() {}

impl Console {
    pub fn new(command_table: HashMap<String, fn()>, prompt: String) -> Console {
        Console {
            command_table,
            prompt,
        }
    }
    pub fn run_repl(&self) {
        loop {
            let input = io::input(&self.prompt);
        }
    }
    pub fn parse(&self, input: String) -> Result<(), Box<dyn Error>> {
        let mut in_string = false;
        let mut tokenized_output: (fn(), Vec<String>) = (nothing, vec![]);
        for (index, item) in input.split(' ').enumerate() {
            if index == 0 {
                tokenized_output.0 = self.command_table[item];
            } else if FoundChar::new(item, '"').occurences == 1 && in_string == true{
                let string_to_append = tokenized_output.1[tokenized_output.1.len()].clone();
                tokenized_output.1.pop();
                tokenized_output.1.push(string_to_append.to_owned() + item);
                in_string = false;
            } else if FoundChar::new(item, '"').occurences == 1 {
                in_string = true;
            } else if in_string == true {
                let string_to_append = tokenized_output.1[tokenized_output.1.len()].clone();
                tokenized_output.1.pop();
                tokenized_output.1.push(string_to_append.to_owned() + item);
            } else {
                tokenized_output.1.push(item.to_owned());
            }
        }
        Ok(())
    }
}

struct FoundChar {
    occurences: usize,
    indices: Vec<usize>,
}

impl FoundChar {
    pub fn new(input: &str, to_find: char) -> FoundChar {
        let mut output = FoundChar {
            occurences: 0,
            indices: vec![],
        };

        for (index, char) in input.chars().enumerate() {
            if char == to_find {
                output.occurences += 1;
                output.indices.push(index);
            }
        }
        output
    }
}