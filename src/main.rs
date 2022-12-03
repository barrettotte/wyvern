use std::env;

mod lexer;
mod parser;
mod interpreter;

use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => interpreter.interpret_file(&args[1]),
        _ => println!("Usage: cargo run [file_path]")
    }
}
