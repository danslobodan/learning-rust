fn read_num() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Invalid input");

    let num: u32 = input.trim().parse().expect("Invalid number.");
    num
}

fn read_str() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Invalid input");

    String::from(input.trim())
}

/*
1. Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers. N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.

Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.

Finally, calculate the difference as 225 - 55 = 170.
 */
fn calculate_sq_diff(num: u32) {
    let mut sum = 0;
    let mut sum_square = 0;
    for i in 1..=num {
        sum += i;
        sum_square += i * i;
    }

    let diff = sum * sum - sum_square;

    println!("The difference is {}", diff);
}

/*
2. Write a program to find the sum of natural numbers below a given number N, where N is provided by the user. The sum should only include numbers that are multiples of either 3 or 5.

For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15. Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.

The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
 */
fn calculate_sum_3_5(num: u32) {
    let mut sum = 0;
    for i in 1..num {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("The sum of numbers divisible by 3 or 5 lower than {} is {}", num, sum);
}

/*
3. This question involves writing code to analyze the production of an assembly line in a car factory. The assembly line has different speeds, ranging from 0 (off) to 10 (maximum). At the lowest speed of 1, the assembly line produces a total of 221 cars per hour. The production rate increases linearly with the speed, meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

However, higher speeds increase the likelihood of producing faulty cars that need to be discarded. The success rate depends on the speed, as shown in the table below:

· Speeds 1 to 4: 100% success rate.
· Speeds 5 to 8: 90% success rate.
· Speeds 9 and 10: 77% success rate.

You need to write two functions:

1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. The function takes the number of hours and speed as input and returns the number of cars successfully produced.
2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. The function takes the number of hours and speed as input and returns the number of cars produced per minute.

Write the code for both functions based on the provided specifications.
 */
fn success_rate(speed: u32) -> f32 {
    match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9 | 10 => 0.8,
        _ => 0.0,
    }
}

fn total_production(hours: u32, speed: u32) -> f32 {
    (hours as f32) * (speed as f32) * success_rate(speed) * 221.0
}

fn cars_produced_per_minute(hours: u32, speed: u32) -> f32 {
    total_production(hours, speed) / 60.0
}

/*
4. A palindrome is a word, verse, or sentence that reads the same backward or forward, such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not. The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
 */
fn is_palindrome(word: &String) -> bool {
    let mut chars = word.chars();
    let mut reversed = word.chars().rev();

    loop {
        let ch = chars.next();
        let rev_ch = reversed.next();

        if ch == None && rev_ch == None {
            break;
        }

        if ch != rev_ch {
            return false;
        }
    }

    true
}

/*
5. A Pythagorean triple consists of three positive integers a, b, and c, satisfying the condition a^2 + b^2 = c^2. These triples are commonly written as (a, b, c), and a well-known example is (3, 4, 5).

Write a program that computes the Pythagorean triplet such that a < b < c and a + b + c = 1000
 */
fn pythagorean() -> Vec<(i32, i32, i32)> {
    let mut results: Vec<(i32, i32, i32)> = Vec::new();

    for i in 1..1000 {
        let jmax = 1000 - i;
        for j in i..jmax {
            let k = 1000 - i - j;
            let pyth = ((i as f32).powf(2.0) + (j as f32).powf(2.0)).sqrt();

            if pyth == (k as f32) {
                results.push((i, j, k));
            }
        }
    }

    results
}

/*
6. Write a function that implements the logic, 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'

Use the following skeleton of the function. Remove the 'return false' statement once you have written the code inside the function.

fn can_see_movie(age: i32, permission: bool) -> bool {

    // Write your code here

    // Remove 'return false' once you have written the code

    return false;

}
 */
fn can_see_movie(age: u32, permission: bool) -> bool {
    match age {
        1..=12 => false,
        13..=16 => permission,
        _ => true,
    }
}
