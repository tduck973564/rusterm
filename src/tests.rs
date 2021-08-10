#[cfg(test)]
use crate::scan;
#[cfg(test)]
use crate::lex;

#[test]
fn scan() {
    assert_eq!(scan::scan("hh \"hello world\"hhhhh hello\"hello there \" \"hello\"".to_string()), vec!["hh", "hello world", "hhhhh", "hello", "hello there ", "hello"])
}

#[test]
fn lex() {
    for (right, left) in lex::lex(scan::scan("hh 43 \"Hello, world!\"".to_string())).iter().zip(vec![lex::Arguments::String("hh".to_string()), lex::Arguments::Number(43), lex::Arguments::String("Hello, world!".to_string())]) {
        if right != &left {
            panic!("")
        }
    }
}