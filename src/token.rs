#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

#[derive(Debug)]
pub enum TokenType {
    Invite,
    NewLine,
    CarriageReturn,
    HorizontalTab,
    LeftAngleBracket,
    RightAngleBracket,
    Text,
    Colon,
    SemiColon,
    Eof,
}
