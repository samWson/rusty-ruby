use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::Peekable;
use std::str::Chars;

mod token;

use token::Token;
/// Rusty Ruby
/// This program is the first stage of a Ruby interpreter. A ruby file is provided as an
/// argument and tokenized.
/// `$ cargo run simple.rb`
/// If the program is run without an argument it will enter Read Eval Print loop.

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
                        let token = lookup_identifier(ident_literal);
                        tokens.push(token);
                    }
                    '=' => {
                        tokens.push(Token::Assign(ch.to_string()));
                        characters.next();
                    }
                    '0'...'9' => {
                        let integer_literal = read_number(&mut characters);
                        tokens.push(Token::Integer(integer_literal));
                        characters.next();
                    }
                    '+' => {
                        tokens.push(Token::Plus(ch.to_string()));
                        characters.next();
                    }
                    '(' => {
                        tokens.push(Token::LParen(ch.to_string()));
                        characters.next();
                    }
                    ')' => {
                        tokens.push(Token::RParen(ch.to_string()));
                        characters.next();
                    }
                    ',' => {
                        tokens.push(Token::Comma(ch.to_string()));
                        characters.next();
                    }
                    ' ' | '\n' => {
                        // Whitespace is ignored.
                        characters.next();
                    }

                    _ => {
                        tokens.push(Token::Illegal(ch.to_string()));
                        characters.next();
                    }
                },
                None => break, // No more chars to tokenize.
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
                }
                _ => break, // Reached the end of the identifier.
            },
            None => break,
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
                }
                _ => break, // Reached the end of the number.
            },
            None => break,
        }
    }
    number
}

fn lookup_identifier(identifier: String) -> Token {
    match identifier.as_ref() {
        "return" => Token::Return("return".to_string()),
        "def" => Token::Method("def".to_string()),
        "end" => Token::End("end".to_string()),
        _ => Token::Ident(identifier.to_string()),
    }
}

#[test]
fn test_lookup_identifier() {
    let test_input = vec!["return".to_string(), "add".to_string()].into_iter();

    let expected_results = vec![
        Token::Return("return".to_string()),
        Token::Ident("add".to_string()),
    ].into_iter();

    let test_cases = test_input.zip(expected_results);

    for (input, expected_result) in test_cases {
        assert_eq!(lookup_identifier(input), expected_result);
    }
}

#[test]
fn test_read_identifier() {
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
TEN = 10

def add(x, y)
  return x + y
end
"
        .to_string();

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
        Token::Method("def".to_string()),
        Token::Ident("add".to_string()),
        Token::LParen("(".to_string()),
        Token::Ident("x".to_string()),
        Token::Comma(",".to_string()),
        Token::Ident("y".to_string()),
        Token::RParen(")".to_string()),
        Token::Return("return".to_string()),
        Token::Ident("x".to_string()),
        Token::Plus("+".to_string()),
        Token::Ident("y".to_string()),
        Token::End("end".to_string()),
    ].into_iter();

    let tokens = input.tokenize().into_iter();

    let test_cases = tokens.zip(test_conditions);

    for (index, (actual, expected)) in test_cases.enumerate() {
        assert_eq!(
            actual,
            expected,
            "\nTokenizing failed at test {}. Expected {:?}, got {:?}",
            index + 1,
            expected,
            actual
        );
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
