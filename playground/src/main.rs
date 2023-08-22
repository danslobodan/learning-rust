fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s2;
    let s4 = &s3;

    let areSame = ***s4 == "Hello".to_string();
}
