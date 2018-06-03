use std::env;
use std::fs::File;
use std::io::prelude::*;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use lexer::Lexer;

/// Rusty Ruby
/// This program is the first stage of a Ruby interpreter. A ruby file is provided as an
/// argument and tokenized.
/// `$ cargo run simple.rb`
/// If the program is run without an argument it will enter Read Eval Print loop.
fn main() {
    let args: Vec<String> = env::args().collect();

    // Enter a REPL if there are no program arguments
    if args.len() == 1 {
        println!("Rusty Ruby Repl");
        repl::start_repl();
        std::process::exit(1);
    }

    // Read the ruby file into the buffer
    let filename = &args[1];
    let mut file = File::open(filename).expect(&format!("Error opening file: {}", filename));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect(&format!("Error reading file: {}", filename));

    println!("{} contents: {:?}", filename, buffer);

    let lexer = Lexer::new(&buffer);

    for token in lexer {
        println!("{:?}", token);
    }
}
