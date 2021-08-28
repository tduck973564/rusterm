/*
 * Copyright (c) 2021 Thomas Duckworth <tduck973564@gmail.com>.
 * This file is under the `rusterm` project, which is licensed under the GNU GPL v3.0 which you can read here: https://www.gnu.org/licenses/gpl-3.0.en.html
 */

#![deny(missing_docs)]
use doc_comment::doc_comment;
doc_comment!(include_str!("../README.md"));

use colored::Colorize as Colourise; // CORRECT ENGLISH!!!
use rustyline::{error::ReadlineError, Cmd, Editor, KeyEvent};
use std::collections::HashMap;
use std::error::Error;

pub mod error;
pub mod lex;
pub mod prelude;
mod scan;
mod tests;

/// The type for Commands passed into the Console. All functions passed in must be of type `fn(brc::lex::Arguments) -> Result<(), brc::error::Error>`
pub type Command = fn(lex::Arguments) -> Result<(), error::Error>;

/// The main constructor owning the console. It contains the command table and the prompt string.
pub struct Console {
    /// Used for aliasing function pointers with names to be looked up later when their alias is typed in to the prompt.
    pub command_table: HashMap<String, Command>,
    /// The characters that come before input, like `>> ` or `Console -> `.
    pub prompt: String,
}

impl Console {
    /// Creates a new instance of the Console struct. It takes two arguments, the command table and the prompt string.
    pub fn new(command_table: HashMap<String, Command>, prompt: &str) -> Console {
        Console {
            command_table,
            prompt: prompt.to_owned(),
        }
    }
    /// Runs the Read, Execute and Print Loop. It displays a prompt where the user can input the command they want, to then be read and parsed.
    pub fn run_repl(&self) {
        loop {
            let mut rl = Editor::<()>::new();

            rl.bind_sequence(KeyEvent::ctrl('A'), Cmd::HistorySearchForward);
            rl.bind_sequence(KeyEvent::ctrl('B'), Cmd::HistorySearchBackward);

            if rl.load_history(".rusterm_history").is_err() {
                eprintln!("Could not load history file.");
            }

            let input = match rl.readline(&self.prompt) {
                Ok(x) if x.is_empty() => continue,
                Ok(x) if x == *"exit" => break,
                Ok(x) => {
                    rl.add_history_entry(x.clone());
                    x
                }
                Err(ReadlineError::Interrupted) => {
                    eprintln!("^C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    eprintln!("^D");
                    break;
                }
                Err(x) => {
                    eprintln!(
                        "{}{} {}",
                        "Error while reading input".red().bold(),
                        ":".bold(),
                        x
                    );
                    break;
                }
            };
            if let Err(x) = self.parse(input) {
                eprintln!("{}", x);
            }
            if rl.append_history(".rusterm_history").is_err() {
                eprintln!("Could not append to history file.");
            }
        }
    }
    fn parse(&self, input: String) -> Result<(), Box<dyn Error>> {
        let mut scanned_input = scan::scan(input);
        let function_name = scanned_input.get(0).unwrap_or(&"".to_string()).clone();
        if !function_name.is_empty() {
            scanned_input.remove(0);
        }
        let lexed_input = lex::lex(scanned_input);
        let function = match function_name {
            x if x == *"help" => {
                self.help();
                return Ok(());
            }
            _ => self
                .command_table
                .get(&function_name)
                .ok_or(error::Error::NoSuchCommand)?,
        };
        if let Err(x) = function(lexed_input) {
            println!("{}", x)
        }
        Ok(())
    }

    fn help(&self) {
        println!("{}", "List of commands:".bold());
        for name in self.command_table.keys() {
            println!("{}", name);
        }
        println!("help");
        println!("exit")
    }
}
