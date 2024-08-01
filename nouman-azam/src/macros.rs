macro_rules! our_macro {
    () => { 1 + 1 };
    (something 4 u dear u32 @ _ @) => {
        println!("You found nonsense here");
    };
    ($e1:expr, $e2:expr) => {
        $e1 + $e2
    };
    ($a:expr, $b:expr; $c:expr) => {
        $a * ($b + $c)
    };
}

pub fn basic_examples() {
    our_macro!();
    println!("{}", our_macro!());
    our_macro!(something 4 u dear u32 @ _ @);
    println!("{}", our_macro!(2, 2));
    println!("{}", our_macro!(3, 4; 5));

    // Will make the macro not compile
    // println!("{}", our_macro!("Something", 4; "nothing"));

    our_macro!();
    our_macro![];
}

macro_rules! input {
    ($t:ty) => {
        {
            let mut n = String::new();
            std::io::stdin()
            .read_line(&mut n)
            .expect("failed to read input");

            let n: $t = n.trim().parse().expect("invalid input");

            n
        }
    };
}

macro_rules! add_as {
    ($a:expr, $b:expr, $typ:ty) => {
        $a as $typ + $b as $typ
    };
}

macro_rules! identifier_macro {
    ($var:ident) => {
        $var = $var + 1;
    };
}

pub fn capturing_types() {
    println!("Please enter a float point number");
    let some_input = input!(f32);
    println!("The input was {}", some_input);

    println!("{}", add_as!(15, 2.3, f32));

    let mut x = 4;
    identifier_macro!(x);
}

macro_rules! create_function {
    ($func_name:ident, $input:ident, $type_input:ty, $type_output:ty) => {
        fn $func_name($input:$type_input) -> $type_output {
            println!("You called {:?}({:?}) with the input of {:?}", stringify!($func_name), stringify!($input), $input);
            $input
        }
    };
}

create_function!(f1, x, i32, i32);

pub fn function_templates() {
    let y = f1(15);
    println!("{}", y);
}

macro_rules! string_concat {
    ($($some_str:expr),*) => {
        {
            let mut temp_str = String::new();
            $(temp_str.push_str($some_str);)*
            temp_str
        }
    };
}

pub fn repeating_patterns() {
    let str_null = string_concat!();
    let str_single = string_concat!("First");
    let str_double = string_concat!("First", "Second");
}
