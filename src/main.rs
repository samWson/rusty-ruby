use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod lexer;
mod token;

use lexer::Lexer;

/// Rusty Ruby
/// This program is the first stage of a Ruby interpreter. A ruby file is provided as an
/// argument and tokenized.
/// `$ cargo run simple.rb`
/// If the program is run without an argument it will enter Read Eval Print loop.

fn start_repl() -> () {
    let scanner = io::stdin();
    let mut input = String::new();

    loop {
        match scanner.read_line(&mut input) {
            Ok(_) => {
                let tokens = input.tokenize();
                for token in tokens {
                    println!("{:?}", token);
                }
            }
            Err(error) => {
                println!("error: {}", error);
                return;
            }
        }
        input = "".to_string(); // Clear the buffer
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Enter a REPL if there are no program arguments
    if args.len() == 1 {
        println!("Rusty Ruby Repl");
        start_repl();
        std::process::exit(1);
    }

    // Read the ruby file into the buffer
    let filename = &args[1];
    let mut file = File::open(filename).expect(&format!("Error opening file: {}", filename));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect(&format!("Error reading file: {}", filename));

    println!("{} contents: {:?}", filename, buffer);

    let tokens = buffer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}
