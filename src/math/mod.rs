pub struct Divisor {}

impl Divisor {
    pub fn is_divisible(x: i32, divisor: i32) -> bool {
        if divisor == 0 {
            panic!("Division by zero")
        }
        x % divisor == 0
    }
}

//Alternative location
pub fn is_divisible(x: i32, divisor: i32) -> bool {
    if divisor == 0 {
        panic!("Division by zero")
    }
    x % divisor == 0
}

//Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_divisible_then_true() {
        let actual = Divisor::is_divisible(2, 1);
        let expected = true;
        assert_eq!(actual, expected)
    }

    #[test]
    fn when_not_divisible_then_false() {
        let actual = Divisor::is_divisible(3, 2);
        let expected = false;
        assert_eq!(actual, expected)
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn when_division_by_zero_then_panic() {
        let actual = Divisor::is_divisible(1, 0);
        let expected = false;
        assert_eq!(actual, expected)
    }

    #[test]
    fn when_divisible_then_true_alternative() {
        let actual = is_divisible(2, 1);
        let expected = true;
        assert_eq!(actual, expected)
    }

    #[test]
    fn when_not_divisible_then_false_alternative() {
        let actual = is_divisible(3, 2);
        let expected = false;
        assert_eq!(actual, expected)
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn when_division_by_zero_then_panic_alternative() {
        let actual = is_divisible(1, 0);
        let expected = false;
        assert_eq!(actual, expected)
    }
}
