use std::cmp::Ordering;
use std::num::ParseIntError;

pub fn parse_guess(input: &str) -> Result<u32, ParseIntError> {
    input.trim().parse()
}

pub fn check_guess(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_too_low() {
        assert_eq!(check_guess(10, 50), Ordering::Less);
    }

    #[test]
    fn guess_too_high() {
        assert_eq!(check_guess(90, 50), Ordering::Greater);
    }

    #[test]
    fn guess_correct() {
        assert_eq!(check_guess(50, 50), Ordering::Equal);
    }

    #[test]
    fn parse_valid_number() {
        assert_eq!(parse_guess("42\n"), Ok(42));
    }

    #[test]
    fn parse_rejects_text() {
        assert!(parse_guess("abc").is_err());
    }

    #[test]
    fn parse_rejects_negative() {
        assert!(parse_guess("-5").is_err());
    }

    #[test]
    fn parse_accepts_boundaries() {
        assert_eq!(parse_guess("100"), Ok(100));
        assert_eq!(parse_guess("1"), Ok(1));
    }

    #[test]
    #[should_panic]
    fn example_of_expected_panic() {
        let v: Vec<u32> = vec![];
        let _ = v[0];
    }
}
