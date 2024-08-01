pub fn examples() {
    let some_values = vec!["123", "some1", "some(123)", "abc", "53"];
    for value in some_values {
        println!("{:?}", parse_str(value));
    }

    println!("Call from main with result equals to {:?}", division(9.0, 3.0));
    println!("Call from main with result equals to {:?}", division(0.0, 3.0));
    println!("Call from main with result equals to {:?}", division(9.0, 0.0));

    let result = operations(0.0);
    println!("{:?}", result);
}

use std::num::ParseIntError;

// question mark with Result
fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let integer = input.parse::<i32>()?;

    println!("The value is {:?} is integer {:?}", input, integer);

    Ok(integer)
}

// question mark with Option
fn division(divident: f64, divisor: f64) -> Option<f64> {
    let answer = match divisor {
        0.0 => None,
        _ => Some(divident / divisor),
    };

    let correct = answer?;

    println!("This line will not print in the case of error {:?}", correct);
    Some(correct)
}

#[derive(Debug)]
enum MathError {
    LogErrorNonPositiveLogarithm,
    SqrtErrorNegativeSquareRoot,
}

type MathResult = Result<(), MathError>;

fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::SqrtErrorNegativeSquareRoot)
    } else {
        println!("Sqrt of {} is {}", x, x.sqrt());
        Ok(())
    }
}

fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::LogErrorNonPositiveLogarithm)
    } else {
        println!("Log of {} is {}", x, x.ln());
        Ok(())
    }
}

fn operations(x: f64) -> MathResult {
    sqrt(x)?;
    ln(x)?;
    Ok(())
}
