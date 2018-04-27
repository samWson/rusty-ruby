use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::iter::Peekable;
use std::str::Chars;

/// Rusty Ruby
/// This program is the first stage of a Ruby interpreter. A ruby file is provided as an
/// argument and tokenized.
/// `$ cargo run simple.rb`
/// If the program is run without an argument it will enter Read Eval Print loop.

/// Token is an individual part or word of a programming language. It identifies the type of token and it's literal value as a String.
#[derive(Debug, PartialEq)]
enum Token {
    Ident(String),
    Assign(String),
    Integer(String),
    Plus(String),
    Illegal(String),
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
                    'a'...'z' | 'A'...'Z' => {
                        let ident_literal = read_identifier(&mut characters);
                        tokens.push(Token::Ident(ident_literal));
                        characters.next();
                    },
                    '=' => {
                        tokens.push(Token::Assign(ch.to_string()));
                        characters.next();
                    },
                    '0'...'9' => {
                        let integer_literal = read_number(&mut characters);
                        tokens.push(Token::Integer(integer_literal));
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
                    _ => {
                        tokens.push(Token::Illegal(ch.to_string()));
                        characters.next();
                    },
                },
                None => break // No more chars to tokenize.
            }
        }
        tokens
    }
}

fn read_identifier(characters: &mut std::iter::Peekable<std::str::Chars>) -> String {
   let mut ident = String::new();

    loop {
        match characters.peek() {
            Some(&ch) => match ch {
                'a'...'z' | 'A'...'Z' => {
                    ident.push(ch);
                    characters.next();
                },
                _ => break, // Reached the end of the identifier.
            },
            None => break
        }
    }
    ident
}

fn read_number(characters: &mut Peekable<Chars>) -> String {
    let mut number = String::new();

    loop {
        match characters.peek() {
            Some(&ch) => match ch {
                '0'...'9' => {
                    number.push(ch);
                    characters.next();
                },
                _ => break, // Reached the end of the number.
            },
            None => break
        }
    }
    number
}

#[test]
fn test_read_identifier(){
    let mut characters = "five = 5".chars().peekable();

    let expected = "five".to_string();

    let actual = read_identifier(&mut characters);

    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize() {
    let input = r"x = 2 + 3
~`
five = 5
TEN = 10".to_string();

    let test_conditions = vec![
        Token::Ident("x".to_string()),
        Token::Assign("=".to_string()),
        Token::Integer("2".to_string()),
        Token::Plus("+".to_string()),
        Token::Integer("3".to_string()),
        Token::Illegal("~".to_string()),
        Token::Illegal("`".to_string()),
        Token::Ident("five".to_string()),
        Token::Assign("=".to_string()),
        Token::Integer("5".to_string()),
        Token::Ident("TEN".to_string()),
        Token::Assign("=".to_string()),
        Token::Integer("10".to_string()),
    ].into_iter();

    let tokens = input.tokenize().into_iter();

    let test_cases = tokens.zip(test_conditions);

    for (index, (actual, expected)) in test_cases.enumerate() {
        assert_eq!(actual, expected, "\nTokenizing failed at test {}. Expected {:?}, got {:?}", index + 1, expected, actual);
    }
}

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
                return
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
    file.read_to_string(&mut buffer).expect(&format!("Error reading file: {}", filename));

    println!("{} contents: {:?}", filename, buffer);

    let tokens = buffer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}
