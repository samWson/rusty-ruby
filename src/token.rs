/// Token is an individual part or word of a programming language. It identifies the type of token and it's literal value as a String.
#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Assign(String),
    Integer(String),
    Plus(String),
    Illegal(String),
    Method(String),
    LParen(String),
    RParen(String),
    Comma(String),
    Return(String),
    End(String),
}
