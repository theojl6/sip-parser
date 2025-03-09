use crate::sip::{Sip, SipMethod};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

//
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Sip {
        Sip {
            method: SipMethod::Invite,
        }
    }

    // method Request-uri sip-version
    fn request_line(&mut self) {}
}
