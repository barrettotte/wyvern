use crate::token::{Token};

pub struct Parser {
    tokens: Vec<Token>,
    // symbol table?
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
        }
    }

    // parse tokens to AST
    pub fn parse(&mut self, tokens: &Vec<Token>) {
        // build AST
    }
}
