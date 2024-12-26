mod lexer;
mod types;
mod parser;
mod interpreter;
use std::io::{self, Write};
use crate::types::{PyType, Stmt, Expr};


use types::{print_env, Environment};

fn main() {
    println!("TomPython Version 1.0");

    let stdin = io::stdin();
    let mut env: Environment = Vec::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap(); // Required for Rust to print string without newline character

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // let input = input.trim();

        // Exit condition
        if input == "q" || input == "quit" {
            break;
        }
        
        execute(&input, &mut env);
        
    }

    println!("Goodbye...");

}

fn execute(input: &str, env: &mut Environment) {
    let mut prev_indent = 0; // could change to curr_indent that tracks current indent, not always pushed to stack
    let mut indent_stack = Vec::<i32>::new(); // should be able to replace prev_indent and original_indent

    match lexer::tokenize(input, &mut prev_indent) {
        Ok((t, indentation)) => {
            // print!("Tokens: ");
            // for token in &t {
            //     print!("{}, ", token)
            // }
            // println!("");
            prev_indent = indentation;
            match parser::parse(&t, &mut prev_indent, &mut indent_stack) {
                Ok((tokens_res, expr)) => {
                    match tokens_res[..] {
                        [] => {
                            // print!("Parse Tree: {}\n", expr);
                            match interpreter::evaluate(&expr, env) {
                                // Ok(result) => println!("{}", result),
                                Ok(PyType::Expr(result)) => println!("{}", result),
                                Ok(_) => print!(""), // print!("{}", result), // PyType::Stmt (print nothing)
                                Err(e) => println!("{}", e)
                            }
                        },
                        _ => {
                            println!("SyntaxError: invalid syntax");
                            print!("Tokens: ");
                            for token in &tokens_res {
                                print!("{}, ", token)
                            }
                            println!("");
                        }
                        
                    }
                    // print!("Parse Tree: {}\n", expr);
                    // print_env(env);
                    
                },
                Err(e) => println!("{}", e)
            }
            },
        Err(e) => println!("{}", e)
    }
}

