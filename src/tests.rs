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
    for (right, left) in lex::lex(scan::scan("hh 43 \"Hello, world!\"".to_string()))
        .0
        .iter()
        .zip(vec![
            lex::Argument::String("hh".to_string()),
            lex::Argument::Integer(43),
            lex::Argument::String("Hello, world!".to_string()),
        ])
    {
        assert_eq!(right, &left);
    }
}
