use std::iter::Peekable;
use std::str::Chars;

use std; // Reference the std namspace in the root of the module i.e. main.rs.
use token::Token;

/// Lexer produces the tokens of a given input.
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let token: Option<Token>;

        match self.input.peek() {
            Some(&ch) => match ch {
                'a'...'z' | 'A'...'Z' => {
                    let ident_literal = read_identifier(&mut self.input);
                    token = Some(lookup_identifier(ident_literal));
                }
                '=' => {
                    token = Some(read_assign_or_equal(&mut self.input));
                }
                '0'...'9' => {
                    let integer_literal = read_number(&mut self.input);
                    token = Some(Token::Integer(integer_literal));
                }
                '+' => {
                    token = Some(Token::Plus(ch.to_string()));
                    self.input.next();
                }
                '-' => {
                    token = Some(Token::Minus(ch.to_string()));
                    self.input.next();
                }
                '*' => {
                    token = Some(Token::Asterisk(ch.to_string()));
                    self.input.next();
                }
                '/' => {
                    token = Some(Token::Slash(ch.to_string()));
                    self.input.next();
                }
                '!' => {
                    token = Some(read_bang_or_not_equal(&mut self.input));
                }
                '<' => {
                    token = Some(Token::LessThan(ch.to_string()));
                    self.input.next();
                }
                '>' => {
                    token = Some(Token::GreaterThan(ch.to_string()));
                    self.input.next();
                }
                '(' => {
                    token = Some(Token::LParen(ch.to_string()));
                    self.input.next();
                }
                ')' => {
                    token = Some(Token::RParen(ch.to_string()));
                    self.input.next();
                }
                ',' => {
                    token = Some(Token::Comma(ch.to_string()));
                    self.input.next();
                }
                ' ' | '\n' => {
                    token = Some(Token::Whitespace(ch.to_string()));
                    self.input.next();
                }
                _ => {
                    token = Some(Token::Illegal(ch.to_string()));
                    self.input.next();
                }
            },
            None => {
                self.input.next();
                return None;
            }
        }

        return token;
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
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

/// read_assign_or_equal returns either an assignment or equality token.
fn read_assign_or_equal(characters: &mut Peekable<Chars>) -> Token {
    let mut literal = characters
        .next()
        .expect("Tried to read a '=', got None instead.")
        .to_string();

    let token: Token;
    if let Some(&'=') = characters.peek() {
        literal.push('=');
        token = Token::Equal(literal);
        characters.next();
    } else {
        token = Token::Assign(literal);
    }
    token
}

// read_bang_or_not_equal returns either a bang or not-equal token.
fn read_bang_or_not_equal(characters: &mut Peekable<Chars>) -> Token {
    let mut literal = characters
        .next()
        .expect("Tried to read a '!', got None instead.")
        .to_string();

    let token: Token;
    if let Some(&'=') = characters.peek() {
        literal.push('=');
        token = Token::NotEqual(literal);
        characters.next();
    } else {
        token = Token::Bang(literal);
    }
    token
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

// FIXME: This test really needs a better name to reflect its context.
#[test]
fn test_read_bang_or_not_equal_single() {
    let mut input = "!(5 < 10)".chars().peekable();
    let expected = Token::Bang("!".to_string());

    let token = read_bang_or_not_equal(&mut input);

    assert_eq!(token, expected);
}

// FIXME: This test really needs a better name to reflect its context.
#[test]
fn test_read_bang_or_not_equal_double() {
    let mut input = "!= 5".chars().peekable();
    let expected = Token::NotEqual("!=".to_string());

    let token = read_bang_or_not_equal(&mut input);

    assert_eq!(token, expected);
}

#[test]
fn test_read_assign_or_equal_single() {
    let mut input = "= 8".chars().peekable();
    let expected = Token::Assign("=".to_string());

    let token = read_assign_or_equal(&mut input);

    assert_eq!(token, expected);
}

#[test]
fn test_read_assign_or_equal_double() {
    let mut input = "== 8".chars().peekable();
    let expected = Token::Equal("==".to_string());

    let token = read_assign_or_equal(&mut input);

    assert_eq!(token, expected);
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
end

10 == 10
10 != 9"
        .to_string();

    let test_conditions = vec![
        Token::Ident("x".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Assign("=".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("2".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Plus("+".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("3".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Illegal("~".to_string()),
        Token::Illegal("`".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Ident("five".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Assign("=".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("5".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Ident("TEN".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Assign("=".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("10".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Method("def".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Ident("add".to_string()),
        Token::LParen("(".to_string()),
        Token::Ident("x".to_string()),
        Token::Comma(",".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Ident("y".to_string()),
        Token::RParen(")".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Return("return".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Ident("x".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Plus("+".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Ident("y".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::End("end".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Ident("result".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Assign("=".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Ident("add".to_string()),
        Token::LParen("(".to_string()),
        Token::Ident("five".to_string()),
        Token::Comma(",".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Ident("TEN".to_string()),
        Token::RParen(")".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Bang("!".to_string()),
        Token::Minus("-".to_string()),
        Token::Slash("/".to_string()),
        Token::Asterisk("*".to_string()),
        Token::Integer("4".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Ident("boolean_result".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Assign("=".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Bang("!".to_string()),
        Token::LParen("(".to_string()),
        Token::Integer("5".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::LessThan("<".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("10".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::GreaterThan(">".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("5".to_string()),
        Token::RParen(")".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::If("if".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::LParen("(".to_string()),
        Token::Integer("5".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::LessThan("<".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("10".to_string()),
        Token::RParen(")".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Return("return".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::True("true".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Else("else".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Return("return".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::False("false".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::End("end".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Integer("10".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Equal("==".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("10".to_string()),
        Token::Whitespace('\n'.to_string()),
        Token::Integer("10".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::NotEqual("!=".to_string()),
        Token::Whitespace(' '.to_string()),
        Token::Integer("9".to_string()),
    ];

    let mut lexer = Lexer::new(&input);
    let mut tokens: Vec<Token> = vec![];
    while let Some(token) = lexer.next() {
        tokens.push(token);
    }

    // The iterator produced by zip() is the size of the smallest of the two zipped iterators.
    // This can mean some of the last test conditions will not be tested for, or some of the last
    // produced tokens will not be tesed, and the test will still pass if the produced tokens and
    // test conditions are not equal in length.
    let expected_tokens_count = test_conditions.len();
    let mut tests_completed = 0;
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

        tests_completed += 1;
    }

    assert_eq!(
        expected_tokens_count, tests_completed,
        "\nNumber of produced tokens is not equal to the expected number. Expected {}, got {}.",
        expected_tokens_count, tests_completed
    );
}
