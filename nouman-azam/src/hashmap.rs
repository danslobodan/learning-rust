fn examples() {
    let mut person: HashMap<&str, i32> = HashMap::new();
    person.insert("Slobodan Dan", 39);
    person.insert("Petar Petrovic", 20);
    person.insert("Jelena Jelenovic", 50);

    println!("The age is {:?}", person.get("Slobodan Dan").unwrap());

    if person.contains_key("Slobodan Dan") {
        println!("The value exists");
    } else {
        println!("The value does not exist");
    }

    match person.get("Slobodan Dan") {
        Some(value) => println!("The value exists {}", value),
        None => println!("The value does not exist"),
    }

    for (name, age) in &person {
        println!("The person {} has an age of {}", name, age);
    }

    let mut fruits: HashMap<&str, &str> = HashMap::new();

    fruits.insert("Slobodan", "Watermellon");
    fruits.insert("Slobodan", "Lemon");

    println!("Slobodan likes {}", fruits.get("Slobodan").unwrap());

    fruits.entry("Slobodan").or_insert("Papaya");

    println!("Slobodan likes {}", fruits.get("Slobodan").unwrap());

    number_frequency();
}

use std::{ collections::HashMap, hash::Hash };

fn number_frequency() {
    let some_vec = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5];
    let mut freq_map: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq = freq_map.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("Frequencies {:?}", freq_map);
}
