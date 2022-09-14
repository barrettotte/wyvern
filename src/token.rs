use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Lambda,
    Dot,
    Variable,
    Assign,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    tok_type: TokenType,
    line: usize,
    col: usize,
    lexeme: Vec<char>,
}

impl Token {
    pub fn new(tok_type: TokenType, line: usize, col: usize, lexeme: Vec<char>) -> Token {
        Token {
            tok_type,
            line,
            col,
            lexeme,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lexeme: String = self.lexeme.iter().collect();
        let len = if lexeme == "λ" {
            1
        } else {
            lexeme.len()
        };

        write!(f, "Token{{tok_type=\"{}\", line={}, col={}:{}, lexeme=\"{}\"}}", 
            self.tok_type, self.line, self.col + 1, self.col + len, lexeme)
    }
}