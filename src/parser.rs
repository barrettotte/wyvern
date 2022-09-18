use std::{fmt, io::Error};
use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Application {
        left: Box<AstNode>,
        right: Box<AstNode>
    },
    Abstraction {
        param: Box<AstNode>,
        body: Box<AstNode>
    },
    Definition {
        identifier: Box<AstNode>,
        body: Box<AstNode>
    },
    Identifier(String),
    EOF,
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize
    // symbol table?
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            idx: 0
        }
    }

    // parse tokens to AST
    pub fn parse(&mut self, tokens: Vec<Token>) -> AstNode {
        self.tokens = tokens;
        self.idx = 0;

        let root = self.parse_statement();
        self.consume(&TokenType::EOF);
        return root;
    }

    // view next token to be consumed
    fn peek(&mut self) -> Result<&Token, String> {
        match self.tokens.get(self.idx) {
            Some(t) => Ok(t),
            None => Err(String::from("Out of tokens"))
        }
    }

    // consume token of certain type or panic
    fn consume(&mut self, tok_type: &TokenType) -> &Token {
        let t = self.peek();
        if t.is_type(*tok_type) {
            panic!("Parser error, unexpected token. Expected {:?}, but found {:?}", *t.get_type(), tok_type);
        }
        self.idx += 1;
        return t;
    }

    fn parse_statement(&mut self) -> AstNode {
        let t = self.peek();
        if t.is_type(TokenType::EOF) {
            return AstNode::EOF;
        } else if t.is_type(TokenType::Identifier) {

        }
        panic!("Parser error, unexpected token. Expected {:?}, but found {:?}", t.get_type(), TokenType::Identifier)
    }
}
