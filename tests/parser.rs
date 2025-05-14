use std::fs::read_to_string;

use stoa::{diagnostic::Diagnostic, lexer::lex, parser::{self, parse}};


#[test]
fn basic_quotes() {
    let code = read_to_string("./tests/fixtures/basic_quotes.stoa").unwrap();
    let mut diags = Vec::<Diagnostic>::new();
    let tokens = lex(&code, &mut diags).unwrap();

    let store = parse(&tokens, &mut diags).unwrap();

    println!("{}", store.pretty_string())

}

#[test]
fn basic_macro() {
    let code = read_to_string("./tests/fixtures/basic_macro.stoa").unwrap();
    let mut diags = Vec::<Diagnostic>::new();
    let tokens = lex(&code, &mut diags).unwrap();
    let store = parse(&tokens, &mut diags).unwrap();

    println!("{}", store.pretty_string())

}
