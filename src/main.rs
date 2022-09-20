use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

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
        let buff = BufReader::new(f);
        let mut lexer = lexer::Lexer::new();
        let mut parser = parser::Parser::new();

        for line in buff.lines() {
            match line {
                Ok(src) => {
                    println!("{}", src);
                    match lexer.lex_line(&src) {
                        Ok(tokens) => {
                            for t in tokens.iter() {
                                println!("  {}", t);
                            }
                            match parser.parse(tokens) {
                                Ok(ast) => println!("{}", ast),
                                Err(e) => panic!("Parser error: {:?}", e)
                            }
                            println!();
                        },
                        Err(e) => panic!("Lexer error: {:?}", e)
                    }
                },
                Err(e) => panic!("Unable to read line. {:?}", e)
            }
        }
    } else {
        println!("Could not find file {}", path);
    }
}
