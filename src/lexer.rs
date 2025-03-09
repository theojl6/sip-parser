use crate::token::{Token, TokenType};

// Scans through each character and groups keywords, syntax
pub struct Lexer {
    source: Vec<char>,
    tokens: Option<Vec<Token>>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: None,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        self.tokens = Some(vec![]);
        self.start = 0;
        self.current = 0;
        self.line = 1;
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.as_mut().unwrap().push(Token {
            token_type: TokenType::Eof,
            lexeme: "EOF".to_string(),
        });
        dbg!(&self.tokens);
        self.tokens.take().expect("Something unexpected happened")
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '<' => {
                self.add_token(TokenType::LeftAngleBracket);
            }
            '>' => {
                self.add_token(TokenType::RightAngleBracket);
            }
            ':' => {
                self.add_token(TokenType::Colon);
            }
            ';' => {
                self.add_token(TokenType::SemiColon);
            }
            '\n' => {
                self.add_token(TokenType::NewLine);
                self.line = self.line + 1;
            }
            '\r' => {
                self.add_token(TokenType::CarriageReturn);
            }
            '\t' => {
                self.add_token(TokenType::HorizontalTab);
            }
            c if !is_separator(c) => {
                self.identifier();
            }
            _ => {}
        }
    }

    fn peek(&self) -> char {
        self.source[self.current]
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current = self.current + 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .as_mut()
            .unwrap()
            .push(Token { token_type, lexeme })
    }

    fn identifier(&mut self) {
        while !self.is_at_end() && !is_separator(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        match text.to_ascii_lowercase().as_str() {
            "invite" => {
                self.add_token(TokenType::Invite);
            }
            "via" => {
                self.add_token(TokenType::Via);
            }
            "max-forwards" => {
                self.add_token(TokenType::MaxForwards);
            }
            _ => {
                self.add_token(TokenType::Text);
            }
        }
        self.start = self.current;
    }
}

fn is_separator(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\r' || c == '\t' || c == ':' || c == '<' || c == '>'
}
