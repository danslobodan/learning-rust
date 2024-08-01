pub fn example() {
    concatenation();
}

fn string_literals() {
    let str =
        r#"We can put "double quotes" in a string if we use an "r" prefix and a hash symbol at the begging and the end."#;
    println!("{}", str);

    let str =
        r"We don't need the hash sign if we don't have double quotes inside our string. \n \t ' ";
    println!("{}", str);

    let json_str = r#"{
        "name" : "Michael",
        "age" : 40,
        "sex" : "Male"
    }"#;
    println!("{}", json_str);

    let str =
        r##"To show the # (hash) symbol inside the string, write double hash symbol at the begging and the end."##;
    println!("{}", str);
}

fn concatenation() {
    let s1 = String::from("Hello");
    let s2 = "World";

    // memory pointing to s1 is changed to store "Hello World" string
    // s3 takes ownership of s1, that is the memory s1 pointed to
    let s3 = s1 + s2;

    // println!("{}", &s1); - won't compile
    println!("{}", &s3);

    let s1 = String::from("Hello");
    let s2 = String::from("World");

    // memory pointing to s1 is changed to store "Hello World" string
    // s3 takes ownership of s1, that is the memory s1 pointed to
    // s2 has not changed ownership
    let s3 = s1 + &s2;
    println!("{} {}", s3, s2);
}
