use std::fs;
use std::error::Error;

fn main() {
    let text = fs::read_to_string("logs.txt");

    println!("{:#?}", text);
}
