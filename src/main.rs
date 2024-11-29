mod lexer;
mod types;
use std::io::{self, Write};

fn main() {
    println!("Welcome to my interpreter!");

    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Required for Rust to print string without newline character

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        // Exit condition
        if input == "q" || input == "quit" {
            break;
        }
        
        println!("{input}");
        match lexer::tokenize(input) {
            Ok(tokens) => println!("Success"),
            Err(e) => println!("{}", e)
        }
    }

    println!("Goodbye...");

}
