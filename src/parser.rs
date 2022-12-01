use std::{fmt};
use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Application {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>
    },
    Abstraction {
        parameter: Box<AstNode>,
        term: Box<AstNode>
    },
    Definition {
        identifier: Box<AstNode>,
        term: Box<AstNode>
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
        let mut lhs = self.parse_atom();
        loop {
            let rhs = self.parse_atom();

            if rhs.is_none() {
                return lhs.unwrap();
            } else {
                lhs = Some(Ok(AstNode::Application {
                    lhs: Box::new(lhs.unwrap().unwrap()),
                    rhs: Box::new(rhs.unwrap().unwrap())
                }))
            }
        }
    }

    fn parse_abstraction(&mut self) -> ParseResult {
        self.consume(TokenType::Lambda)?;
        let identifier = self.parse_identifier()?;
        self.consume(TokenType::Dot)?;
        let term = self.parse_term()?;

        return Ok(AstNode::Abstraction { 
            parameter: Box::new(identifier),
            term: Box::new(term)
        })
    }

    fn parse_atom(&mut self) -> Option<ParseResult> {
        match self.peek() {
            Ok(token) => {
                match token.get_type() {
                    TokenType::OpenParen => Some(self.parse_parenthetical_term()),
                    TokenType::Identifier => Some(self.parse_identifier()),
                    _ => None
                }
            },
            Err(e) => Some(Err(format!("Error parsing atom. {}", e))),
        }
    }

    fn parse_parenthetical_term(&mut self) -> ParseResult {
        self.consume(TokenType::OpenParen)?;
        let term = self.parse_term();
        self.consume(TokenType::CloseParen)?;
        return term
    }

    fn parse_term(&mut self) -> ParseResult {
        match self.peek()?.get_type() {
            TokenType::EOF => Ok(AstNode::EOF),
            TokenType::Lambda => self.parse_abstraction(),
            _ => self.parse_application()
        }
    }

    fn parse_definition(&mut self, identifier: AstNode) -> ParseResult {
        self.consume(TokenType::Assign)?;
        Ok(AstNode::Definition {
            identifier: Box::new(identifier), 
            term: Box::new(self.parse_term()?)
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
                        self.parse_term()
                    }
                }
            },
            _ => self.parse_term()
        }
    }
}
