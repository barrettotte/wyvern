use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => run_file(&args[1]),
        _ => println!("Usage: cargo run [file_path]")
    }
}

fn run_file(path: &str) {
    if let Ok(f) = File::open(path) {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let buff = BufReader::new(f);

        for line in buff.lines() {
            match line {
                Ok(src) => {
                    println!("{}", src);
                    process_line(&mut lexer, &mut parser, src)
                },
                Err(e) => panic!("Unable to read line. {:?}", e)
            }
        }
    } else {
        println!("Could not find file {}", path);
    }
}

fn process_line(lexer: &mut Lexer, parser: &mut Parser, src: String) {
    match lexer.lex_line(&src) {
        Ok(tokens) => {
            for t in tokens.iter() {
                println!("  {}", t);
            }
            match parser.parse(tokens) {
                Ok(ast) => {
                    println!("{}", ast);
                    // TODO: reduce
                    // TODO: evaluate AST
                },
                Err(e) => panic!("Parser error: {:?}", e)
            }
            println!();
        },
        Err(e) => panic!("Lexer error: {:?}", e)
    }
}

// TODO: tests
