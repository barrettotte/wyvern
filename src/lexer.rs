use std::{fmt,str};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Lambda,
    Dot,
    Identifier,
    OpenParen,
    CloseParen,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
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

    pub fn get_type(&self) -> TokenType {
        self.tok_type
    }

    pub fn get_lexeme(&self) -> &Vec<char> {
        &self.lexeme
    }

    pub fn is_type(&self, t: TokenType) -> bool {
        self.tok_type == t
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.lexeme.iter().collect::<String>();
        write!(f, "Token{{tok_type=\"{}\", line={}, col={}:{}, lexeme=\"{}\"}}", 
            self.tok_type, self.line, self.col + 1, self.col + s.len(), s)
    }
}

type LexResult = Result<Option<Token>, String>;

pub struct Lexer {
    src: Vec<char>,
    col: usize,
    line: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            src: Vec::new(),
            col: 0,
            line: 1,
        }
    }

    // reset internal state of lexer
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.src = Vec::new();
        self.col = 0;
        self.line = 1;
    }

    // scan a single line of source to tokens
    pub fn lex_line(&mut self, src: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        self.col = 0;
        self.src = src.chars().collect();

        while !self.is_eol() {
            if let Some(t) = self.lex_token()? {
                tokens.push(t);
            }
        }
        tokens.push(self.new_token(TokenType::EOF, self.col).unwrap().unwrap());
        self.advance_line();
        Ok(tokens)
    }

    fn advance(&mut self) {
        self.col += 1;
    }

    fn advance_line(&mut self) {
        self.advance(); // consume newline
        self.line += 1;
        self.col = 0;
    }

    fn is_eol(&self) -> bool {
        self.col >= self.src.len()
    }

    fn is_identifier_char(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn is_ignore_char(&self, c: char) -> bool {
        c == '(' || c == ')' || c.is_whitespace()
    }

    fn peek(&self) -> Result<char, String> {
        match self.src.get(self.col) {
            Some(c) => Ok(*c),
            None => Err(String::from("Out of characters"))
        }
    }

    fn new_token(&self, tok_type: TokenType, start: usize) -> LexResult {
        let valid_range = self.src.get(start).is_some() && self.src.get(self.col-1).is_some();

        if valid_range || tok_type == TokenType::EOF {
            return Ok(Some(Token::new(tok_type, self.line, start, self.src[start..self.col].to_vec())));
        }
        return Err(format!("Failed to resolve lexeme for token type {}. Line {}, {}:{}", 
            tok_type, self.line, start, self.col));
    }

    fn lex_comment(&mut self) -> LexResult {
        while !self.is_eol() {
            self.advance();
        }
        Ok(None)
    }

    fn lex_identifier(&mut self) -> LexResult {
        let start = self.col;
        while !self.is_eol() && self.is_identifier_char(self.peek()?){
            self.advance();
        }
        return self.new_token(TokenType::Identifier, start);
    }

    fn lex_single(&mut self, tok_type: TokenType) -> LexResult {
        self.advance();
        self.new_token(tok_type, self.col - 1)
    }

    fn lex_token(&mut self) -> LexResult {
        match self.peek()? {
            'λ' | '\\' => self.lex_single(TokenType::Lambda),
            '.' => self.lex_single(TokenType::Dot),
            '(' => self.lex_single(TokenType::OpenParen),
            ')' => self.lex_single(TokenType::CloseParen),
            '#' => self.lex_comment(),
            c if self.is_identifier_char(c) => self.lex_identifier(),
            c if self.is_ignore_char(c) => {
                self.advance();
                Ok(None)
            },
            _ => Err(format!("Bad character. Line {}, column {}", self.line, self.col))
        }
    }
}
