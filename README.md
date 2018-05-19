# Rusty Ruby
Learn how Ruby works by implementing it in Rust.

## Description
A good way to understand how something works is to make it. The goal of this project is to implement a Ruby interpreter using the Rust programming language.

### Interpreting Ruby

Source code for any language goes through several stages before any instructions are executed. A *lexer* analyses the source code and produces tokens, which are the *parts* of the language. A *parser* will parse the tokens into an *abstract syntax tree* or *AST* which represents the syntax of a program. An *interpreter* will then interpret the AST and execute the instructions required by the program.

![Flow of a Ruby program through an interpreter](https://github.com/samWson/rusty-ruby/blob/master/ruby-interpreter-flow-chart.jpg)

## Dependencies
Make sure you have [Rust](https://www.rust-lang.org/en-US/install.html) installed.

## Usage
The program is an attempt to do the first stage of interpreting a Ruby program: Tokenization. A ruby file provided as an argument and the program outputs the resulting tokens.
```
$ cargo run sample_ruby.rb
```
If the program is run without arguments it will enter a Read Eval Print loop. Arbitrary code can be entered at the command line and the tokens will be produced as output.

### Limitations
Currently only the most basic syntax will be supported. The example program `sample_ruby.rb` gives an indication of what can be tokenized. As the supported syntax grows more expressions can be added to `sample_ruby.rb`.
```
x = 2 + 3
five = 5
TEN = 10

def add(x, y)
  return x + y
end

result = add(five, TEN)

boolean_result = !(5 < 10 > 5)

if (5 < 10)
  return true
else
  return false
end

10 == 10
10 != 9
```
Note that it is not the responsibility of the Tokenizer to spot syntax errors. Any unrecognized character will be output as an illegal token.

## Code Layout

The source code is broken down into separate files based on responsibility.

### Main

`main.rs` is the entry point of the program.

### Lexer

`lexer.rs` is responsible for lexical analysis. A lexer scans each character in Ruby source code and produces appropriate tokens based on the values it scans.

### REPL

`repl.rs` is responsible for the Read Eval Print Loop. This can evaluate arbitrary code from the standard input, evaluate it, and print the result to the standard output. Currently it only outputs tokens.

### Token

`token.rs` is responsible for defining the tokens of the Ruby language. A token is a part of a language such as keywords, operators, identifiers etc.

## Licence
This project is Open Sourced under the MIT licence. Although this project is personal learning exercise, it has been made using company time at [Flux Federation](http://fluxfederation.com/) during our hack days. All copyright and IP belongs to Flux Federation.

## Contributions
Contributions are welcome as long as the contributor is happy with the licencing and copyright as described above.
