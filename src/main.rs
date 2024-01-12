fn fizzbuzz(number: u8) -> String {
    match (number % 3, number % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        _ => number.to_string(),
    }
}

fn main() {
    (1..=100)
        .map(fizzbuzz)
        .for_each(|result| println!("{}", result));
}

#[cfg(test)]
mod fizzbuzz_tests {
    use super::*;

    #[test]
    fn test_multiple_of_three_is_fizz() {
        assert_eq!(fizzbuzz(3), "fizz");
        assert_eq!(fizzbuzz(9), "fizz");
    }

    #[test]
    fn test_multiple_of_five_is_buzz() {
        assert_eq!(fizzbuzz(5), "buzz");
        assert_eq!(fizzbuzz(10), "buzz");
    }

    #[test]
    fn test_multiple_of_three_and_five_is_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "fizzbuzz");
        assert_eq!(fizzbuzz(45), "fizzbuzz");
    }

    #[test]
    fn test_number_not_divisible_by_three_or_five_is_number() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(4), "4");
        assert_eq!(fizzbuzz(98), "98");
    }
}
