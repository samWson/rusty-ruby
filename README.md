# Rusty Ruby
Learn how Ruby works by implementing it in Rust.

## Description
A good way to understand how something works is to make it. The goal of this project is to implement a Ruby interpreter using the Rust programming language.

## Usage
The program can currently do the first stage of interpreting a Ruby program: Tokenization. A ruby file provided as an argument and the program outputs the resulting tokens.
```
$ cargo run simple.rb
```

### Limitations
Currently only the most basic syntax will be supported. The example program `simple.rb` gives an indication of what can be tokenized.
```
10.times do |n|
  puts n
end
```

## Licence
This project is Open Sourced under the MIT licence. Athough this project is personal learning exercise, it has been made using company time at [Flux Federation](http://fluxfederation.com/) during our hack days. All copyright and IP belongs to Flux Federation.

## Contributions
Contributions are welcome as long as the contributor is happy with the licencing and copyright as described above.
