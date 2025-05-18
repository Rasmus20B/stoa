use std::{fs::read_to_string, path::PathBuf};

use stoa_core::{diagnostic::Diagnostic, lexer::lex, parser::{self, parse}};


#[test]
fn basic_quotes() {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("basic_quotes.stoa");

    let code = read_to_string(fixture_path).unwrap();
    let mut diags = Vec::<Diagnostic>::new();
    let tokens = lex(&code, &mut diags).unwrap();

    let store = parse(&tokens, &mut diags).unwrap();

    println!("{}", store.pretty_string())

}

#[test]
fn basic_macro() {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("basic_macro.stoa");

    let code = read_to_string(fixture_path).unwrap();
    let mut diags = Vec::<Diagnostic>::new();
    let tokens = lex(&code, &mut diags).unwrap();
    let store = parse(&tokens, &mut diags).unwrap();

    println!("{}", store.pretty_string())

}
