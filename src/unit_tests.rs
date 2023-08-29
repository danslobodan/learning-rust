pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn division(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        panic!("Division by zero!");
    } else {
        x / y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(3));
    }

    #[test]
    fn two_numbers_are_equal() {
        assert_eq!(2, 2);
    }

    #[test]
    #[should_panic]
    fn division_by_zero() {
        division(5.0, 0.0);
    }

    #[test]
    fn simple_addition() -> Result<(), String> {
        if 2 + 2 == 4 { Ok(()) } else { Err(String::from("The two values are not equal")) }
    }

    #[test]
    #[ignore]
    fn long_running_test() {
        for num1 in 0..1_000_000 {
            for num2 in 0..1_000_000 {
                let multiplied = num1 * num2;
                let multiplied_back = num2 * num1;
                assert_eq!(multiplied, multiplied_back);
            }
        }
    }
}
