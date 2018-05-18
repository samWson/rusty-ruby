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
                    }
                    '+' => {
                        tokens.push(Token::Plus(ch.to_string()));
                        characters.next();
                    }
                    '-' => {
                        tokens.push(Token::Minus(ch.to_string()));
                        characters.next();
                    }
                    '*' => {
                        tokens.push(Token::Asterisk(ch.to_string()));
                        characters.next();
                    }
                    '/' => {
                        tokens.push(Token::Slash(ch.to_string()));
                        characters.next();
                    }
                    '!' => {
                        tokens.push(Token::Bang(ch.to_string()));
                        characters.next();
                    }
                    '<' => {
                        tokens.push(Token::LessThan(ch.to_string()));
                        characters.next();
                    }
                    '>' => {
                        tokens.push(Token::GreaterThan(ch.to_string()));
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
                'a'...'z' | 'A'...'Z' | '_' => {
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
        "if" => Token::If("if".to_string()),
        "else" => Token::Else("else".to_string()),
        "true" => Token::True("true".to_string()),
        "false" => Token::False("false".to_string()),
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

result = add(five, TEN)
!-/*4
boolean_result = !(5 < 10 > 5)

if (5 < 10)
  return true
else
  return false
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
        Token::Ident("result".to_string()),
        Token::Assign("=".to_string()),
        Token::Ident("add".to_string()),
        Token::LParen("(".to_string()),
        Token::Ident("five".to_string()),
        Token::Comma(",".to_string()),
        Token::Ident("TEN".to_string()),
        Token::RParen(")".to_string()),
        Token::Bang("!".to_string()),
        Token::Minus("-".to_string()),
        Token::Slash("/".to_string()),
        Token::Asterisk("*".to_string()),
        Token::Integer("4".to_string()),
        Token::Ident("boolean_result".to_string()),
        Token::Assign("=".to_string()),
        Token::Bang("!".to_string()),
        Token::LParen("(".to_string()),
        Token::Integer("5".to_string()),
        Token::LessThan("<".to_string()),
        Token::Integer("10".to_string()),
        Token::GreaterThan(">".to_string()),
        Token::Integer("5".to_string()),
        Token::RParen(")".to_string()),
        Token::If("if".to_string()),
        Token::LParen("(".to_string()),
        Token::Integer("5".to_string()),
        Token::LessThan("<".to_string()),
        Token::Integer("10".to_string()),
        Token::RParen(")".to_string()),
        Token::Return("return".to_string()),
        Token::True("true".to_string()),
        Token::Else("else".to_string()),
        Token::Return("return".to_string()),
        Token::False("false".to_string()),
        Token::End("end".to_string()),
    ];

    let tokens = input.tokenize();

    // The iterator produced by zip() is the size of the smallest of the two zipped iterators.
    // This can mean some of the last test conditions will not be tested for, or some of the last
    // produced tokens will not be tesed, and the test will still pass if the produced tokens and
    // test conditions are not equal in length.
    let expected_tokens_count = test_conditions.len();
    let tokens_count = tokens.len();

    assert_eq!(
        expected_tokens_count, tokens_count,
        "\nNumber of produced tokens is not equal to the expected number. Expected {}, got {}.",
        expected_tokens_count, tokens_count
    );

    let test_cases = tokens.into_iter().zip(test_conditions);

    for (index, (actual, expected)) in test_cases.into_iter().enumerate() {
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
