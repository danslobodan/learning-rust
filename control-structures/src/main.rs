fn main() {
    for_loop();
}

fn if_else_statement() {
    let marks = 65;
    if marks >= 60 && marks <= 70 {
        println!("The grade is satisfactory");
    }

    let flag1 = true;
    let flag2 = false;

    if flag1 == true || flag2 == true {
        println!("One of the flags is true.");
    }

    if flag1 == true && (flag2 != true || marks < 50) {
        println!("The usage of parenthesis.");
    } else if flag1 == true && marks == 100 {
        println!("Alternative condition");
    } else {
        println!("The else block");
    }

    // let if
    let value = if true { "It's true" } else { "It's false" };
    println!("{}", value);
}

fn match_statement() {
    let marks = 65;
    let mut grade = 'N';
    match marks {
        90..=100 => {
            grade = 'A';
        }
        80..=89 => {
            grade = 'B';
        }
        70..=79 => {
            grade = 'C';
        }
        60..=69 => {
            grade = 'D';
        }
        _ => {
            grade = 'F';
        }
    }
    println!("The grade is {}", grade);

    // let match
    let grade = match marks {
        90..=100 => {
            grade = 'A';
        }
        80..=89 => {
            grade = 'B';
        }
        70..=79 => {
            grade = 'C';
        }
        60..=69 => {
            grade = 'D';
        }
        _ => {
            grade = 'F';
        }
    };
    println!("The grade is {:?}", grade);
}

fn while_loop() {
    let my_number = 5;
    let mut guess = false;

    println!("Guess my number which is between 1 and 20");

    while guess != true {
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).expect("Failed to read input.");

        let number: u8 = number.trim().parse().expect("Invalid input.");

        if my_number == number {
            println!("You guessed the number correctly.");
            guess = true;
        } else {
            println!("Please try again.");
        }
    }
}

fn for_loop() {
    let vec = vec![45, 30, 85, 90, 41, 39];

    // for
    for i in 0..=5 {
        println!("Index: {} Value: {}", i, vec[i]);
    }

    // for each
    for n in vec {
        println!("Direct {}", n);
    }

    // for each moves the ownership to the for loop
    // println!("{:?}", vec); - won't compile

    let mut vec = vec![45, 30, 85, 90, 41, 39];

    // iterating without moving ownership
    for n in vec.iter() {
        println!("Iterator {}", n);
    }
    println!("{:?}", vec);

    for n in &vec {
        println!("Borrow {}", n);
    }
    println!("{:?}", vec);

    // mutable vector value references
    for n in vec.iter_mut() {
        *n += 5;
        println!("Mutable iterator {}", n);
    }
    println!("{:?}", vec);

    for n in &mut vec {
        *n += 5;
        println!("Mutable borrow {}", n);
    }
    println!("{:?}", vec);
}
