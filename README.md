# Rust Fizzbuzz Implementation

This is an example implementation of the classic FizzBuzz kata in Rust, written test-first.

## Acceptance criteria

- The program, when run, should count from 1 to 100, printing to the screen.
- If the number is divisible by 3, instead of printing the number, the word "fizz" should be printed.
- If the number is divisible by 5, instead of printing the number, the word "buzz" should be printed.
- If the number is divisible by 3 and 5, instead of printing the number, the word "fizzbuzz" should be printed.

## Tests

Run the tests with:

```shell-session
cargo test
```

## Run

Run the program with:

```shell-session
cargo run
```
## Design considerations

Rust’s expressive pattern matching and type system encourage a type-driven approach that encodes each possible outcome as a distinct enum variant. By doing so, we avoid dealing with brittle string matching or manipulation.

The objective is to showcase Rust’s emphasis on explicit, type-safe design, even in a trivial kata.

Matching on an enum variant is more robust than matching on strings, because it eliminates potential errors from typos or unexpected text.

Enumerations also provide a straightforward way to add new cases, like a modulo 7 variant, without disrupting existing logic.

Using Display to render each variant cleanly separates domain logic from presentation concerns. 



