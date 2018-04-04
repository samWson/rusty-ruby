extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

/// Rusty Ruby
/// This program is the first stage of a Ruby interpreter. A ruby file is provided as an
/// argument and tokenized.
/// `$ cargo run simple.rb`

/// Token is an individual part or word of a programming language. It identifies the type of token and it's literal value as a String.
#[derive(Debug, PartialEq)]
enum Token {
    Ident(String),
    Assign(String),
    Integer(String),
    Plus(String),
}

/// Lexer is for a source e.g. String, file etc. into the individual tokens that make up the parts of a language.
trait Lexer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Lexer for String {
    fn tokenize(&self) -> Vec<Token> {
        let mut characters = self.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        loop {
            match characters.peek() {
                Some(&ch) => match ch {
                    'a'...'z' => {
                        tokens.push(Token::Ident(ch.to_string()));
                        characters.next();
                    },
                    '=' => {
                        tokens.push(Token::Assign(ch.to_string()));
                        characters.next();
                    },
                    '0'...'9' => {
                        tokens.push(Token::Integer(ch.to_string()));
                        characters.next();
                    },
                    '+' => {
                        tokens.push(Token::Plus(ch.to_string()));
                        characters.next();
                    },
                    ' ' | '\n' => {
                        // Whitespace is ignored.
                        characters.next();
                    },
                    _ => panic!("Unrecognized character"),
                },
                None => break // No more chars to tokenize.
            }
        }
        tokens
    }
}

#[test]
fn test_tokenize() {
    let simple_expression = "x = 2 + 3".to_string();

    let expected = vec![
        Token::Ident("x".to_string()),
        Token::Assign("=".to_string()),
        Token::Integer("2".to_string()),
        Token::Plus("+".to_string()),
        Token::Integer("3".to_string())
    ];

    assert_eq!(simple_expression.tokenize(), expected);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // Exit the program if there are no arguments
        println!(r"Usage:
    rr <ruby-file>");
        std::process::exit(1);
    }

    // Read the ruby file into the buffer
    let filename = &args[1];
    let mut file = File::open(filename).expect(&format!("Error opening file: {}", filename));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect(&format!("Error reading file: {}", filename));

    println!("{} contents: {:?}", filename, buffer);

    // These are our individual characters of ruby code, ready for tokenization.
    let mut chars: Vec<char> = Vec::new();
    for char in buffer.chars() {
        chars.push(char);
    }

    // Creating a numeric matcher with regex
    let numeric = Regex::new(r"\d|\.").unwrap();

    for char in chars {
        if numeric.is_match(&char.to_string()) {
            println!("{} is a numeric.", char);
        }
    }
    // println!("Our vec of chars: {:?}", chars); // chars is moved by the for in loop and no longer available.
    // We can now make our collection of chars. It's time to iterate over them and tokenize.
}
