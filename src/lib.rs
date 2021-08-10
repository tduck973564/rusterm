use std::error::Error;
use std::collections::HashMap;
mod io;
mod scan;
mod lex;
mod error;
mod tests;

pub struct Console {
    command_table: HashMap<String, fn(Vec<lex::Arguments>)>,
    prompt: String,
}

impl Console {
    pub fn new(command_table: HashMap<String, fn(Vec<lex::Arguments>)>, prompt: String) -> Console {
        Console {
            command_table,
            prompt,
        }
    }
    pub fn run_repl(&self) {
        loop {
            let input = io::input(&self.prompt);
            if let Err(x) = self.parse(input) {
                eprintln!("{}", x)
            }
        }
    }
    pub fn parse(&self, input: String) -> Result<(), Box<dyn Error>> {
        let mut scanned_input = scan::scan(input);
        let function_name = scanned_input[0].clone();
        scanned_input.remove(0);
        let lexed_input = lex::lex(scanned_input);
        let function = match function_name {
            x if x == "help".to_string() => { self.help(); return Ok(()) },
            _ =>  match self.command_table.get(&function_name) {
                Some(x) => x,
                None => return Err(Box::new(error::Error::NoSuchCommand))
            }
        };
        function(lexed_input);
        Ok(())
    }

    fn help(&self) {
        for (name, _) in &self.command_table {
            println!("{}", name);
        }
    }
}
