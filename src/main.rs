use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u8),
}

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FizzBuzz::FizzBuzz => write!(f, "FizzBuzz"),
            FizzBuzz::Fizz => write!(f, "Fizz"),
            FizzBuzz::Buzz => write!(f, "Buzz"),
            FizzBuzz::Number(n) => write!(f, "{}", n.to_string()),
        }
    }
}

fn fizzbuzz(number: u8) -> FizzBuzz {
    match (number % 3, number % 5) {
        (0, 0) => FizzBuzz::FizzBuzz,
        (0, _) => FizzBuzz::Fizz,
        (_, 0) => FizzBuzz::Buzz,
        _ => FizzBuzz::Number(number),
    }
}

fn main() {
    (1..=100).for_each(|number| println!("{}", fizzbuzz(number)));
}

#[cfg(test)]
mod fizzbuzz_tests {
    use super::*;

    #[test]
    fn test_multiple_of_three_is_fizz() {
        assert_eq!(fizzbuzz(3), FizzBuzz::Fizz);
        assert_eq!(fizzbuzz(9), FizzBuzz::Fizz);
    }

    #[test]
    fn test_multiple_of_five_is_buzz() {
        assert_eq!(fizzbuzz(5), FizzBuzz::Buzz);
        assert_eq!(fizzbuzz(10), FizzBuzz::Buzz);
    }

    #[test]
    fn test_multiple_of_three_and_five_is_fizzbuzz() {
        assert_eq!(fizzbuzz(15), FizzBuzz::FizzBuzz);
        assert_eq!(fizzbuzz(45), FizzBuzz::FizzBuzz);
    }

    #[test]
    fn test_number_not_divisible_by_three_or_five_is_number() {
        assert_eq!(fizzbuzz(1), FizzBuzz::Number(1));
        assert_eq!(fizzbuzz(4), FizzBuzz::Number(4));
        assert_eq!(fizzbuzz(98), FizzBuzz::Number(98));
    }
}
