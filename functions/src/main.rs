fn main() {
    let full_name = "Slobodan Dan";
    let salary = 10_000;
    print_salary(full_name, salary);
    let result = multiply(2, 3);
    println!("Multiplication result {}", result);

    let (addition, subtraction, multiplication) = multi_output(4, 3);
    println!("{} + {} = {}", 4, 3, addition);
    println!("{} - {} = {}", 4, 3, subtraction);
    println!("{} * {} = {}", 4, 3, multiplication);

    let full_name = {
        let first_name = "Slobodan";
        let last_name = "Dan";
        format!("{} {}", first_name, last_name)
    };
    println!("My name is {}", full_name);

    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read input");

    let n: f64 = n.trim().parse().expect("invalid input");
    println!("You have entered: {}", n);
}

fn print_salary(name: &str, salary: i32) {
    println!("{} is paid {} euro monthly.", name, salary);
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn multi_output(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 + num2, num1 - num2, num1 * num2)
}
