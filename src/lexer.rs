use crate::token::{Token, TokenType};

pub struct Lexer {
    src: Vec<char>,
    idx: usize,
    col: usize,
    line: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            src: Vec::new(),
            idx: 0,
            col: 0,
            line: 1,
        }
    }

    // scan a single line of source to tokens
    pub fn lex_line(&mut self, src: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        self.src = src.chars().collect();

        while self.peek() != '\n' {
            if let Some(t) = self.lex_token() {
                tokens.push(t);
            }
        }
        self.advance_line();
        return tokens;
    }

    fn advance(&mut self) {
        self.idx += 1;
        self.col += 1;
    }

    fn advance_line(&mut self) {
        self.advance(); // consume newline
        self.line += 1;
        self.col = 0;
    }

    fn peek(&self) -> char {
        self.src[self.idx]
    }

    fn is_eof(&self) -> bool {
        self.idx >= self.src.len()
    }

    fn new_token(&self, tok_type: TokenType, start: usize) -> Token {
        Token::new(tok_type, self.line, self.col, self.src[start..self.idx].to_vec())
    }

    fn lex_comment(&mut self) {
        self.advance(); // start comment
        let c = self.peek();

        // consume characters until next line reached
        while c != '\n' && !self.is_eof() {
            self.advance();
        }
    }

    fn lex_variable(&mut self) -> Token {
        let mut var: Vec<char> = Vec::new();
        let start = self.col;

        while self.peek().is_alphanumeric() && !self.is_eof() {
            var.push(self.peek());
            self.advance();
        }
        self.new_token(TokenType::Variable, start)
    }

    fn lex_assignment(&mut self) -> Token {
        self.advance(); // consume ':'

        if self.peek() == '=' {
            self.new_token(TokenType::Assign, self.idx - 1)
        } else {
            panic!("Bad character, expected '='. Line {}, column {}", self.line, self.col);
        }
    }

    fn lex_token(&mut self) -> Option<Token> {
        match self.peek() {
            '\n' => {
                self.advance_line();
                None
            },
            '#' => {
                self.lex_comment();
                None
            },
            ':' => {
                Some(self.lex_assignment())
            },
            ' ' => {
                self.advance();
                None
            },
            ')' => {
                self.advance();
                None
            },
            '(' => {
                self.advance();
                None
            },
            'λ' => {
                self.advance();
                Some(self.new_token(TokenType::Lambda, self.idx))
            },
            '.' => {
                self.advance();
                Some(self.new_token(TokenType::Dot, self.idx))
            },
            c => {
                if !c.is_alphanumeric() {
                    panic!("Invalid character found. Line {}, column {}", self.line, self.col);
                }
                Some(self.lex_variable())
            }
        }
    }
}
