use std::iter::Peekable;
use std::str::Chars;

use std; // Reference the std namspace in the root of the module i.e. main.rs.
use token::Token;

/// Lexer is for a source e.g. String, file etc. into the individual tokens that make up the parts of a language.
pub trait Lexer {
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
end"
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
