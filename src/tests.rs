// Copyright 2021 Thomas Duckworth <tduck973564@gmail.com>.
// This file is part of the `brc` project, licenced under the GNU GPL v3.0, which can be read here: https://www.gnu.org/licenses/gpl-3.0.en.html

#[cfg(test)]
use crate::prelude::*;
#[cfg(test)]
use crate::lex::lex;
#[cfg(test)]
use crate::scan::scan;

#[test]
fn scan_cmp() {
    assert_eq!(
        scan("hh \"hello world\"hhhhh hello\"hello there \" \"hello\"".to_string()),
        vec![
            "hh",
            "hello world",
            "hhhhh",
            "hello",
            "hello there ",
            "hello"
        ]
    )
}

#[test]
fn lex_cmp() {
    for (right, left) in lex(scan("hh 43 \"Hello, world!\" 3.3".to_string()))
        .iter()
        .zip(vec![
            Argument::String("hh".to_string()),
            Argument::Integer(43),
            Argument::String("Hello, world!".to_string()),
            Argument::Float(3.3),
        ])
    {
        assert_eq!(right, &left);
    }
}

#[test]
fn pop_arg() {
    let mut arguments = vec![Argument::String("Hello, World!".to_string())];
    arguments.pop_arg().unwrap();
    arguments.pop_arg().unwrap_err();
}

#[test]
fn parse() {
    let console = Console::new(HashMap::new(), "");
    console.parse("help \"hello, world\"".to_string()).unwrap();
}