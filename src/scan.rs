// Copyright 2021 Thomas Duckworth <tduck973564@gmail.com>.
// This file is part of the `rusterm` project, licenced under the GNU GPL v3.0, which can be read here: https://www.gnu.org/licenses/gpl-3.0.en.html

//! Module for scanning and splitting up user input to be lexed.

fn push_string_to(string: &mut String, array: &mut Vec<String>) {
    if !string.is_empty() {
        array.push(string.to_string())
    }
    *string = "".to_string();
}

/// Scans user input, and splits it up into substrings.
/// # Examples
/// `hello there "hello world"`
/// becomes
/// ['hello', 'there', 'hello world']
pub fn scan(input: String) -> Vec<String> {
    let mut current = String::new();
    let mut output: Vec<String> = Vec::new();
    let mut in_quote = false;

    for character in input.chars() {
        if in_quote {
            if character == '"' {
                in_quote = false;
                push_string_to(&mut current, &mut output);
            } else {
                current += &character.to_string();
            }
        } else if !in_quote {
            if character == '"' {
                in_quote = true;
                push_string_to(&mut current, &mut output);
            } else if character == ' ' {
                push_string_to(&mut current, &mut output);
            } else {
                current += &character.to_string();
            }
        }
    }
    push_string_to(&mut current, &mut output);

    output
}
