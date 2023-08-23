fn main() {
    let i = 5;
    let result: &i32;
    {
        let j = 10;
        result = greater(&i, &j);
        println!("The larger value is {}", result);
    }
}

fn greater<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if i > j { i } else { j }
}
