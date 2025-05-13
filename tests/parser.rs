use std::fs::read_to_string;

use stoa::{diagnostic::Diagnostic, lexer::lex, parser::{self, parse}};


#[test]
fn basic_quotes() {
    let code = read_to_string("./tests/fixtures/basic_quotes.stoa").unwrap();
    let mut diags = Vec::<Diagnostic>::new();
    let tokens = lex(&code, &mut diags).unwrap();

    let store = parse(&tokens, &mut diags).unwrap();

    for entry in store.entries {
        println!("{entry:?}")
    }

}
