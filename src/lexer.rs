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

    // scan a single line of source to tokens
    pub fn lex_line(&mut self, src: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        self.col = 0;
        self.src = src.chars().collect();

        while !self.is_eol() {
            if let Some(t) = self.lex_token() {
                tokens.push(t);
            }
        }
        self.advance_line();
        return tokens;
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

    fn peek(&self) -> char {
        self.src[self.col]
    }

    fn new_token(&self, tok_type: TokenType, start: usize) -> Token {
        Token::new(tok_type, self.line, start, self.src[start..self.col].to_vec())
    }

    fn lex_comment(&mut self) {
        while !self.is_eol() {
            self.advance();
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.col;
        while !self.is_eol() && self.is_identifier_char(self.peek()) {
            self.advance();
        }
        return self.new_token(TokenType::Variable, start);
    }

    fn lex_assignment(&mut self) -> Token {
        self.advance(); // consume ':'

        if self.peek() == '=' {
            self.advance();
            return self.new_token(TokenType::Assign, self.col - 2);
        } else {
            panic!("Bad character, expected '='. Line {}, column {}", self.line, self.col);
        }
    }

    fn lex_lambda(&mut self) -> Token {
        self.advance();
        return self.new_token(TokenType::Lambda, self.col - 1);
    }

    fn lex_dot(&mut self) -> Token {
        self.advance();
        return self.new_token(TokenType::Dot, self.col - 1);
    }

    fn lex_token(&mut self) -> Option<Token> {
        let mut t: Option<Token> = None;

        match self.peek() {
            'λ' | '\\' => t = Some(self.lex_lambda()),
            '#' => self.lex_comment(),
            ':' => t = Some(self.lex_assignment()),
            '.' => t = Some(self.lex_dot()),
            c => {
                if self.is_identifier_char(c) {
                    t = Some(self.lex_identifier());
                } else if self.is_ignore_char(c) {
                    self.advance();
                } else {
                    panic!("Bad character. Line {}, column {}", self.line, self.col);
                }
            }
        }
        return t;
    }
}
