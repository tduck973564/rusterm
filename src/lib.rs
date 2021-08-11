use std::collections::HashMap;
use std::error::Error;
use rustyline::{ Editor, error::ReadlineError };
use colored::Colorize as Colourise; // CORRECT ENGLISH!!!

pub mod error;
pub mod lex;
mod scan;
mod tests;

pub struct Console {
    command_table: HashMap<String, fn(Vec<lex::Argument>)>,
    prompt: String,
}

impl Console {
    pub fn new(command_table: HashMap<String, fn(Vec<lex::Argument>)>, prompt: String) -> Console {
        Console {
            command_table,
            prompt,
        }
    }
    pub fn run_repl(&self) {
        loop {
            let mut rl = Editor::<()>::new();
            let input = match rl.readline(&self.prompt) {
                Ok(x) => x,
                Err(ReadlineError::Interrupted) => {
                    println!("Exiting...");
                    break;
                },
                Err(ReadlineError::Eof) => {
                    println!("Exiting...");
                    break;
                },
                Err(x) => {
                    println!("{}{} {}", "Error while reading input".red().bold(), ":".bold(), x);
                    break;
                }
            };
            if let Err(x) = self.parse(input) {
                println!("{}", x);
            }
        }
    }
    pub fn parse(&self, input: String) -> Result<(), Box<dyn Error>> {
        let mut scanned_input = scan::scan(input);
        let function_name = scanned_input[0].clone();
        scanned_input.remove(0);
        let lexed_input = lex::lex(scanned_input);
        let function = match function_name {
            x if x == *"help" => {
                self.help();
                return Ok(());
            }
            _ => self.command_table.get_cmd(&function_name)?,
        };
        function(lexed_input);
        Ok(())
    }

    fn help(&self) {
        for name in self.command_table.keys() {
            println!("{}", name);
        }
    }
}

trait CommandGet<K, V> {
    fn get_cmd(&self, input: &K) -> Result<&V, error::Error>;
}

trait ArgGet {
    fn take_arg(&mut self, index: usize) -> Result<lex::Argument, error::Error>;
}

impl CommandGet<String, fn(Vec<lex::Argument>)> for HashMap<String, fn(Vec<lex::Argument>)> {
    fn get_cmd(&self, input: &String) -> Result<&fn(Vec<lex::Argument>), error::Error> {
        self.get(input).ok_or(error::Error::NoSuchCommand)
    }
}
