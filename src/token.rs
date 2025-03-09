#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

#[derive(Debug)]
pub enum TokenType {
    Invite,
    Via,
    MaxForwards,
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
