use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Could not flush stdout");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Could not read from stdin");
    buf
}