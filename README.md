# Rust Fizzbuzz Implementation

This is an example implementation of the classic FizzBuzz kata in Rust.

## Acceptance criteria

- The program, when run, should count from 1 to 100, printing to the screen.
- If the number is divisible by 3, instead of printing the number, the word "fizz" should be printed.
- If the number is divisible by 5, instead of printing the number, the word "buzz" should be printed.
- If the number is divisible by 3 and 5, instead of printing the number, the word "fizzbuzz" should be printed.

## Design considerations

This solution is influenced by my background in Ruby and Erlang.

Rust has expressive pattern matching capabilities, akin to Erlang's `case` expression, and functional-style iterator chaining akin to Ruby's blocks.  I wanted to showcase this functionality.  The tuple matching is particularly elegant.

I elected to return a `String` for each case, since we need to print the result.

## Improvements

Calling `to_string()` on each match arm is a bit inelegant.  An improvement would be to define a `FizzBuzzResult` Enum with variants for each option.  This would also make adding further rules nicely readable

We could then implement the `Display` trait for `FizzBuzzResult`, which would handle string conversion, and would also allow for different output options.
