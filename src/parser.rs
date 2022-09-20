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
        self.consume(TokenType::EOF)?;
        Ok(root?)
    }

    // view next token to be consumed
    fn peek(&mut self) -> Result<&Token, String> {
        match self.tokens.get(self.idx) {
            Some(t) => Ok(t),
            None => Err(String::from("Out of tokens"))
        }
    }

    // consume token of certain type or panic
    fn consume(&mut self, tok_type: TokenType) -> Result<Token, String> {
        let t = self.peek()?.clone();
        if !t.is_type(tok_type) {
            return Err(format!("Unexpected token. Expected {:?}, but found {:?}", t.get_type(), tok_type));
        }
        self.advance();
        Ok(t)
    }

    // advance to next token
    fn advance(&mut self) {
        self.idx += 1
    }

    // rewind to previous token
    fn rewind(&mut self) {
        self.idx -= 1
    }

    fn parse_identifier(&mut self) -> ParseResult {
        Ok(AstNode::Identifier(self.consume(TokenType::Identifier)?.get_lexeme().iter().collect::<String>()))
    }

    fn parse_application(&mut self) -> ParseResult {
        let mut left = match self.peek()?.get_type() {
            TokenType::Identifier => Some(self.parse_identifier()?),
            TokenType::Lambda => Some(self.parse_abstraction()?),
            _ => None
        };
        loop {
            let right = match self.peek()?.get_type() {
                TokenType::Identifier => Some(self.parse_identifier()?),
                TokenType::Lambda => Some(self.parse_abstraction()?),
                _ => None
            };
            match right {
                Some(expr) => {
                    left = Some(AstNode::Application {
                        left: Box::new(left.unwrap()), 
                        right: Box::new(expr)
                    })
                },
                None => return Ok(left.unwrap())
            }
        }
    }

    fn parse_abstraction(&mut self) -> ParseResult {
        self.consume(TokenType::Lambda)?;
        let param = self.parse_identifier()?;
        self.consume(TokenType::Dot)?;
        Ok(AstNode::Abstraction {
            parameter: Box::new(param), 
            expression: Box::new(self.parse_expression()?)
        })
    }

    fn parse_expression(&mut self) -> ParseResult {
        if self.peek()?.get_type() == TokenType::EOF {
            return Ok(AstNode::EOF)
        }
        self.parse_application()
    }

    fn parse_definition(&mut self, identifier: AstNode) -> ParseResult {
        self.consume(TokenType::Assign)?; // consume ':='
        Ok(AstNode::Definition {
            identifier: Box::new(identifier), 
            expression: Box::new(self.parse_expression()?)
        })
    }

    fn parse_statement(&mut self) -> ParseResult {
        match self.peek()?.get_type() {
            TokenType::Identifier => {
                let identifier = self.parse_identifier()?;
                match self.peek()?.get_type() {
                    TokenType::Assign => self.parse_definition(identifier),
                    _ => {
                        self.rewind();
                        self.parse_expression()
                    }
                }
            },
            _ => self.parse_expression()
        }
    }
}
