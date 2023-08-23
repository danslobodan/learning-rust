fn examples() {
    let mut disease: Option<String> = None;
    disease = Some(String::from("Diabetes"));

    match disease {
        Some(disease_name) => println!("You have {}", disease_name),
        None => println!("You are still healthy."),
    }

    let wrapped_value = Some("String value");
    println!("The value of the wrapped variable is {}", wrapped_value.unwrap());
}
