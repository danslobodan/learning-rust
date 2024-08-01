use regex::Regex;

pub fn example() {
    let re = Regex::new(r"[prt]ain").unwrap();
    let text = "rrrain spain none";

    println!("The text has a match {:?}", re.is_match(text));
    println!("The text has a match {:?}", re.find(text));

    for capture in re.captures_iter(text) {
        println!("match: {:?}", &capture[0]);
    }
}
