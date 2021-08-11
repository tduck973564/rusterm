// Copyright 2021 Thomas Duckworth <tduck973564@gmail.com>.
// This file is part of the `brc` project, licenced under the GNU GPL v3.0, which can be read here: https://www.gnu.org/licenses/gpl-3.0.en.html

//! A fast and easy console library.
//! 
//! This provides a basic, minimal framework over making a console for applications. 
//!
//! # Syntax
//! 
//! `<command> [stringargument] ["string argument with spaces"] [33] [4.8]`
//!
//! Note that 33 and 4.8 are two different types, Integer and Float. 
//!
//! # Examples
//!
//! ```
//! use rusterm::prelude::*;
//!
//! fn main() {
//!     let mut command_table: HashMap<String, Command>= HashMap::new();
//!     command_table.insert("add".to_string(), add);
//!     let console = Console::new(command_table, ">> ");
//!     console.run_repl();
//! }
//!
//! fn add(mut args: Arguments) -> Result<(), BrcError> {
//!     let mut sum = 0;
//!     for _ in 0..args.len() {
//!         let arg: i32 = args.pop_arg()?.try_into()?;
//!         sum += arg;
//!     }
//!     println!("{}", sum);
//!     Ok(())
//! }
//! ```
//!
//! # Writing functions that work as commands
//!
//! Every function that you write to be used in the Console must follow this signature: `fn(mut args: brc::lex::Arguments) -> Result<(), brc::error::Error>`.
//! To use user-inputted arguments in your function, you must continually pop the command arguments from the args parameter, and then convert them into the type you expect.
//! Example below:
//!
//! ```
//! use rusterm::prelude::*;
//!
//! fn echo(mut args: Arguments) -> Result<(), BrcError> {
//!     let first_argument: String = args.pop_arg()?.try_into()?; // Expects a String, as per the type annotation. You can expect a f64, i32 or String.
//!     println!("{}", first_argument);
//!     Ok(())
//! }
//! ```
//!

#![deny(missing_docs)]

use colored::Colorize as Colourise; // CORRECT ENGLISH!!!
use rustyline::{error::ReadlineError, Editor};
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
    /// Runs the Read, Execute, Print Loop. It displays a prompt where the user can input the command they want, to then be read and parsed. 
    pub fn run_repl(&self) {
        loop {
            let mut rl = Editor::<()>::new();
            let input = match rl.readline(&self.prompt) {
                Ok(x) if x.is_empty() => continue,
                Ok(x) if x == *"exit" => break,
                Ok(x) => x,
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    break;
                }
                Err(x) => {
                    println!(
                        "{}{} {}",
                        "Error while reading input".red().bold(),
                        ":".bold(),
                        x
                    );
                    break;
                }
            };
            if let Err(x) = self.parse(input) {
                println!("{}", x);
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
            },
            _ => self.command_table.get(&function_name).ok_or(error::Error::NoSuchCommand)?,
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
