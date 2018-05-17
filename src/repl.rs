use std::io;

use lexer::Lexer;

/// start_repl starts a Read Eval Print Loop. Arbitrary code entered as standard input is evaluated
/// and the result is printed to the standard output.
pub fn start_repl() -> () {
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
