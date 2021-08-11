use std::error::Error;
use std::collections::HashMap;
use rustyline;

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
            let mut rl = rustyline::Editor::<()>::new();
            let input = match rl.readline(&self.prompt) {
                Ok(x) => x,
                Err(_) => continue,
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
            x if x == "help".to_string() => { self.help(); return Ok(()) },
            _ => self.command_table.get_cmd(&function_name)?,
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

trait CommandGet<K, V> {
    fn get_cmd(&self, input: &K) -> Result<&V, error::Error>;
}

impl CommandGet<String, fn(Vec<lex::Arguments>)> for HashMap<String, fn(Vec<lex::Arguments>)> {
    fn get_cmd(&self, input: &String) -> Result<&fn(Vec<lex::Arguments>), error::Error> {
        self.get(input).ok_or_else(|| { error::Error::NoSuchCommand } )
    }
}
