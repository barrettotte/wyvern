use std::{fmt};
use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Application {
        left: Box<AstNode>,
        right: Box<AstNode>
    },
    Abstraction {
        parameter: Box<AstNode>,
        expression: Box<AstNode>
    },
    Definition {
        identifier: Box<AstNode>,
        expression: Box<AstNode>
    },
    Identifier(String),
    EOF,
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

type ParseResult = Result<AstNode, String>;

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
    pub fn parse(&mut self, tokens: Vec<Token>) -> ParseResult {
        self.tokens = tokens;
        self.idx = 0;

        let root = self.parse_statement();
        self.consume(TokenType::EOF);
        root?
    }

    // view next token to be consumed
    fn peek(&mut self) -> Result<&Token, String> {
        match self.tokens.get(self.idx) {
            Some(t) => Ok(t),
            None => Err(String::from("Out of tokens"))
        }
    }

    // consume token of certain type or panic
    fn consume(&mut self, tok_type: TokenType) -> Result<&Token, String> {
        let t = self.peek()?;    
        if !t.is_type(tok_type) {
            return Err(format!("Unexpected token. Expected {:?}, but found {:?}", t.get_type(), tok_type));
        }
        self.idx += 1;
        Ok(t)
    }

    fn consume_identifier(&mut self) -> Result<String, String> {
        Ok(self.consume(TokenType::Identifier)?.get_lexeme().iter().collect::<String>())
    }

    fn parse_application(&mut self) -> ParseResult {
        Ok(AstNode::EOF) // TODO:
    }

    fn parse_abstraction(&mut self) -> ParseResult {
        Ok(AstNode::EOF) // TODO:
    }

    fn parse_expression(&mut self) -> ParseResult {
        // (identifier | application | abstraction)
        let t = self.peek()?;
        match t.get_type() {
            TokenType::Identifier => Ok(AstNode::Identifier(identifier))
        }
    }

    fn parse_definition(&mut self, identifier: String) -> ParseResult {
        self.consume(TokenType::Assign)?; // consume ':='
        Ok(AstNode::Definition {
            identifier: Box::new(AstNode::Identifier(identifier)), 
            expression: Box::new(self.parse_expression()?)
        })
    }

    fn parse_statement(&mut self) -> ParseResult {
        match self.peek()?.get_type() {
            TokenType::Identifier => {
                let identifier = self.consume_identifier?;
                match self.peek()?.get_type() {
                    TokenType::Assign => self.parse_definition(identifier),
                    _ => Ok(AstNode::Identifier(identifier))
                }
            },
            TokenType::EOF => Ok(AstNode::EOF),
            _ => self.parse_expression()
        }
    }
}
