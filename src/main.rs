use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run_file(&args[1]),
        _ => println!("Usage: cargo run [file_path]")
    }
}

fn run_file(path: &str) {
    println!("Running {}", path);
    // lexer new
    // parser new

    if let Ok(f) = File::open(path) {
        // lex file to tokens
        // parse tokens to AST
        // beta reduce AST
        // return AST
    } else {
        println!("Could not find file {}", path);
    }
}
