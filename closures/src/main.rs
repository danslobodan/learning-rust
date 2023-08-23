use std::vec;

fn main() {
    function_pointers();
}

fn basic_closure() {
    let x = 5;
    let square = || println!("The square of the variable is {}", x * x);
    square();
}

fn with_input() {
    let x = 5;
    let square = |num: i32| println!("The square of the variable is {}", num * num);
    square(x);

    let y = 11;
    square(y);
}

fn with_output() {
    let square = |num| num * num;
    let x = 5;
    let restult = square(x);
    println!("The result is {}", restult);

    // will not compile because the closure typed are determined on the first call of the closure
    // let y = 5.5;
    // square(y);
}

fn closure_as_input() {
    let division_status = |y: f32| {
        if y != 0.0 { true } else { false }
    };

    division(5.0, 10.0, division_status);
    division(5.0, 0.0, division_status);
}

fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
    if f(y) == true {
        println!("The division result is {}", x / y);
    } else {
        println!("The division is not possible");
    }
}

fn closure_lifetimes() {
    let mut vec_1 = vec![1, 2, 3];

    let mut some_closure = || {
        vec_1.push(35);
    };

    // cannot modify or read here
    some_closure();
    // can modify or read here
    vec_1[1] = 15;
    println!("{:?}", vec_1);

    let mut vec_1 = vec![1, 2, 3];

    let mut some_closure = || {
        let vec_2 = vec_1;
    };

    some_closure();
    // cannot modify or read, because the value is already moved to vec_2
    // println!("{:?}", vec_1);
}

fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn min(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

fn function_pointers() {
    let f = min;
    println!("The minimum of two values is {}", f(2, 3));
    let f = max;
    println!("The maximum of two values is {}", f(2, 3));

    let (my_name, my_age) = (String::from("Slobodan"), 39);
    prints_full_info(prints_name, &my_name, my_age);
}

fn prints_name(name: &str) {
    print!("The name is {}", name);
}

fn prints_full_info(f: fn(&str), some_one: &str, age: i32) {
    f(some_one);
    println!(" and my age is {}", age);
}
