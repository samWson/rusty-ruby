# Rusty Ruby
Learn how Ruby works by implementing it in Rust.

## Description
A good way to understand how something works is to make it. The goal of this project is to implement a Ruby interpreter using the Rust programming language.

## Usage
The program is an attempt to do the first stage of interpreting a Ruby program: Tokenization. A ruby file provided as an argument and the program outputs the resulting tokens.
```
$ cargo run sample_ruby.rb
```
If the program is run without arguments it will enter a Read Eval Print loop. Arbitrary code can be entered at the command line and the tokens will be produced as output.

### Limitations
Currently only the most basic syntax will be supported. The example program `sample_ruby.rb` gives an indication of what can be tokenized. As the supported syntax grows more expression can be added to `sample_ruby.rb`.
```
x = 2 + 3
```

## Licence
This project is Open Sourced under the MIT licence. Athough this project is personal learning exercise, it has been made using company time at [Flux Federation](http://fluxfederation.com/) during our hack days. All copyright and IP belongs to Flux Federation.

## Contributions
Contributions are welcome as long as the contributor is happy with the licencing and copyright as described above.
