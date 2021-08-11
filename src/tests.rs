// Copyright 2021 Thomas Duckworth <tduck973564@gmail.com>.
// This file is part of the `brc` project, licenced under the GNU GPL v3.0, which can be read here: https://www.gnu.org/licenses/gpl-3.0.en.html

#[cfg(test)]
use crate::lex;
#[cfg(test)]
use crate::scan;

#[test]
fn scan() {
    assert_eq!(
        scan::scan("hh \"hello world\"hhhhh hello\"hello there \" \"hello\"".to_string()),
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
fn lex() {
    for (right, left) in lex::lex(scan::scan("hh 43 \"Hello, world!\" 3.3".to_string()))
        .iter()
        .zip(vec![
            lex::Argument::String("hh".to_string()),
            lex::Argument::Integer(43),
            lex::Argument::String("Hello, world!".to_string()),
            lex::Argument::Float(3.3),
        ])
    {
        assert_eq!(right, &left);
    }
}
