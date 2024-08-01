fn examples() {
    println!("{:?}", division(9.0, 3.0));
    println!("{:?}", division(4.0, 0.0));
    println!("{:?}", division(0.0, 2.0));

    let some_vec = vec![5, 5, 3, 1, 5, 9];
    let result = match some_vec.get(5) {
        Some(a) => Ok(a),
        None => Err("Teh value does not exist"),
    };

    println!("The value of the result is {:?}", result);
}

fn division(divident: f32, divisor: f32) -> Result<f32, String> {
    if divisor == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(divident / divisor)
    }
}

fn division_with_match(divident: f32, divisor: i32) -> Result<f32, String> {
    match divisor {
        0 => Err(String::from("Error: Division by zero")),
        _ => Ok(divident / (divisor as f32)),
    }
}
