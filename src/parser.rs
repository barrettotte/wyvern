use std::{fmt, collections::HashMap};
use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Abstraction {
        parameter: Box<AstNode>,
        term: Box<AstNode>
    },
    Application {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>
    },
    Identifier(String),
    Epsilon,
}

impl AstNode {
    pub fn reduce(&self, env: HashMap<String, AstNode>) -> AstNode {
        match self {
            AstNode::Abstraction {parameter, term} => {
                let mut new_env = env.clone();
                
                if let AstNode::Identifier(ref id) = **parameter {
                    if new_env.contains_key(id) {
                        new_env.remove(id);
                    }
                    if let AstNode::Application {ref lhs, ref rhs} = **term {
                        if rhs == parameter && !lhs.free(&**parameter) {
                            return lhs.reduce(new_env);
                        }
                    }
                } else {
                    panic!("Invalid abstraction")
                }
                AstNode::Abstraction {parameter: parameter.clone(), term: Box::new(term.reduce(new_env))}
            },
            AstNode::Application {ref lhs, ref rhs} => {
                if let AstNode::Abstraction {ref parameter, ref term} = **lhs {
                    let mut new_env = env.clone();

                    if let AstNode::Identifier(ref id) = **parameter {
                        new_env.insert(id.clone(), *rhs.clone());
                        return term.reduce(new_env);
                    }
                    panic!("Invalid application")
                }
                AstNode::Application {
                    lhs: Box::new(lhs.reduce(env.clone())),
                    rhs: Box::new(rhs.reduce(env.clone()))
                }
            },
            AstNode::Identifier(id) => {
                match env.get(id) {
                    Some(x) => x.clone(),
                    None => self.clone(),
                }
            },
            AstNode::Epsilon => self.clone(),
        }
    }

    fn free(&self, id: &AstNode) -> bool {
        match self {
            AstNode::Abstraction {parameter, term} => *id != **parameter && term.free(id),
            AstNode::Application {lhs, rhs} => lhs.free(id) || rhs.free(id),
            AstNode::Identifier(_) => self == id,
            _ => false
        }
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AstNode::Application {lhs, rhs} => write!(f, "({} {})", lhs, rhs),
            AstNode::Abstraction {parameter, term} => write!(f, "λ{}.{}", parameter, term),
            AstNode::Identifier(id) => write!(f, "{}", id),
            AstNode::Epsilon => write!(f, "ε")
        }
    }
}

type ParseResult = Result<AstNode, String>;

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize
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
        Ok(self.parse_statement()?)
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

        Ok(AstNode::Abstraction { 
            parameter: Box::new(identifier),
            term: Box::new(term)
        })
    }

    fn parse_atom(&mut self) -> Option<ParseResult> {
        match self.peek() {
            Ok(token) => {
                match token.get_type() {
                    TokenType::Identifier => Some(self.parse_identifier()),
                    TokenType::OpenParen => Some(self.parse_grouped_term()),
                    _ => None
                }
            },
            Err(e) => Some(Err(format!("Error parsing atom. {}", e))),
        }
    }

    fn parse_grouped_term(&mut self) -> ParseResult {
        self.consume(TokenType::OpenParen)?;
        let term = self.parse_term();
        self.consume(TokenType::CloseParen)?;
        return term
    }

    fn parse_epsilon(&mut self) -> ParseResult {
        self.advance();
        Ok(AstNode::Epsilon)
    }

    fn parse_term(&mut self) -> ParseResult {
        match self.peek()?.get_type() {
            TokenType::EOF | TokenType::CloseParen => self.parse_epsilon(),
            TokenType::Lambda => self.parse_abstraction(),
            _ => self.parse_application()
        }
    }

    fn parse_statement(&mut self) -> ParseResult {
        self.parse_term()
    }
}
