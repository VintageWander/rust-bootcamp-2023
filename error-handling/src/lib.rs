//Execise 1
// Make it compile in unit test
// Run tests
// Hint: Convert Option to Result
fn generate_nametag_text(name: String) -> Result<String, String> {
    match name.is_empty() {
        true => Err("`name` was empty; it must be nonempty.".into()),
        false => Ok(format!("Hi! My name is {}", name)),
    }
}
// Exercise 2
// Make it compile in unit test
// Run tests
// Hint: &str to integer conversion by using parse method and return Result
use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

// Exercise 3
// Make it compile in unit test
// Run tests
// Hint: Custom Error
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test for exercise 1
    #[test]
    fn exercise1_should_work() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );

        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }

    /// Test for exercise 2
    #[test]
    fn exercise2_should_work() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(
            parse_number("invalid").map_err(|e| e.to_string()),
            Err("invalid digit found in string".parse().unwrap())
        );
    }

    /// Test for exercise 3
    #[test]
    fn exercise3_should_work() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
