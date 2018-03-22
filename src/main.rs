use std::env;
use std::fs::File;
use std::io::prelude::*;

/// Rusty Ruby
/// This program is the first stage of a Ruby interpreter. A ruby file is provided as an
/// argument and tokenized.
/// `$ cargo run simple.rb`
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // Exit the program if there are no arguments
        println!(r###"Usage:
    rr <ruby-file>"###);
        std::process::exit(1);
    }

    // Read the ruby file into the buffer
    let filename = &args[1];
    let mut file = File::open(filename).expect(&format!("Error opening file: {}", filename));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect(&format!("Error reading file: {}", filename));

    println!("{} contents: {:?}", filename, buffer);
}
