use std::iter;
use token::Token;

struct Parser<'a> {
    lexer: &'a iter::Peekable<Vec<Token>>,
    // current_token: Token,
    // peek_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &iter::Peekable<Token>) -> &Parser {
        let parser = &Parser{lexer: lexer};

        // Read two tokens, so current_token and peek_token are both set.
        // parser.nextToken();
        // parser.nextToken();

        parser
    }
}
