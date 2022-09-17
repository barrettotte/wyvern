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
    println!("Running {}", path);

    if let Ok(f) = File::open(path) {
        let buff = BufReader::new(f);
        let mut lexer = lexer::Lexer::new();
        // let mut parser = parser::Parser::new();

        for line in buff.lines() {
            if let Ok(src) = line {
                println!("{}", src);
                let tokens = lexer.lex_line(&src);
                for t in tokens.iter() {
                    println!("  {}", t);
                }
                
                // TODO: print out and test values
                // let ast = parser.parse(&tokens);
                // add symbols to symbol table?
                println!();
            }
        }
    } else {
        println!("Could not find file {}", path);
    }
}
