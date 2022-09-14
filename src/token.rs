#[derive(Debug, PartialEq)]
pub enum TokenType {
    Lambda,
    Dot,
    Variable,
    Assign,
    EOF,
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