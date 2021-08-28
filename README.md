# Rusterm
[![docs](https://docs.rs/rusterm/badge.svg)](https://docs.rs/rusterm)
[![dependency status](https://deps.rs/crate/rusterm/0.2.1/status.svg)](https://deps.rs/crate/rusterm/0.2.1)
[![build status](https://github.com/tduck973564/rusterm/workflows/Rust/badge.svg)](https://github.com/tduck973564/rusterm/actions)


A fast and easy console library.
 
This provides a basic, minimal framework for making a basic command interpreter over applications. 

# Syntax
 
`<command> [stringargument] ["string argument with spaces"] [33] [4.8]`

Note that 33 and 4.8 are two different types, Integer and Float. 

# Examples

```
use rusterm::prelude::*;

fn main() {
    let mut command_table: HashMap<String, Command>= HashMap::new();
    command_table.insert("add".to_string(), add);
    let console = Console::new(command_table, ">> ");
    console.run_repl();
}

fn add(mut args: Arguments) -> Result<(), RustermError> {
    let mut sum = 0;
    for _ in 0..args.len() {
        let arg: i32 = args.pop_arg()?.try_into()?;
        sum += arg;
    }
    println!("{}", sum);
    Ok(())
}
```

# Writing functions that work as commands

Every function that you write to be used in the Console must follow this signature: `fn(mut args: brc::lex::Arguments) -> Result<(), brc::error::Error>`.
To use user-inputted arguments in your function, you must continually pop the command arguments from the args parameter, and then convert them into the type you expect.
Example below:

```
fn echo(mut args: Arguments) -> Result<(), RustermError> {
    let first_argument: String = args.pop_arg()?.try_into()?; // Expects a String, as per the type annotation. You can expect a f64, i32 or String.
    println!("{}", first_argument);
    Ok(())
}
```
