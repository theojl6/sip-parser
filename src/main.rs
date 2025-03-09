use sip_parser::lexer::Lexer;
use sip_parser::parser::Parser;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("invite.txt").expect("Cannot read from txt file");
    let mut lexer = Lexer::new(contents);
    let tokens = lexer.lex();
    let parser = Parser::new(tokens);
}
