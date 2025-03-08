use crate::token::{Token, TokenType};

// Scans through each character and groups keywords, syntax
struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn lex(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "EOF".to_string(),
        });
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        todo!()
    }

    fn scan_token(&mut self) {}
}
