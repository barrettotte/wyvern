use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::lexer::Lexer;
use crate::parser::{Parser, AstNode};

pub struct Interpreter {
    lexer: Lexer,
    parser: Parser
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            lexer: Lexer::new(),
            parser: Parser::new()
        }
    }

    pub fn interpret_file(&mut self, path: &str) {
        println!("Interpreting {}", path);
        let f = File::open(path).expect(&format!("Could not find file {}", path));

        for line in BufReader::new(f).lines() {
            match line {
                Ok(src) => self.interpret_line(src),
                Err(e) => panic!("Unable to read line. {:?}", e)
            }
        }
    }

    fn interpret_line(&mut self, line: String) {
        match self.lexer.lex_line(&line) {
            Ok(tokens) => {
                match self.parser.parse(tokens) {
                    Ok(ast) => {
                        if ast != AstNode::Epsilon {
                            print!("{}", ast);
                            println!(" => {}", self.evaluate(ast));
                        }
                    },
                    Err(e) => panic!("Parser error: {:?}", e)
                }
            },
            Err(e) => panic!("Lexer error: {:?}", e)
        }
    }

    fn evaluate(&mut self, mut ast: AstNode) -> AstNode {
        loop {
            let new = ast.reduce(HashMap::new());
            if ast == new {
                break;
            }
            ast = new;
        }
        return ast;
    }
}
