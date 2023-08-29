struct Person {
    name: String,
    age: u32,
    salary: u32,
}

impl Person {
    fn new(name: String, age: u32, salary: u32) -> Result<Self, String> {
        if name.is_empty() {
            Err(String::from("Person name cannot be empty."))
        } else {
            Ok(Person { name, age, salary })
        }
    }

    fn compute_taxes(&self) -> f32 {
        ((self.salary as f32) / 3.0) * 0.5
    }
}

impl Default for Person {
    fn default() -> Self {
        Person { name: String::from("Petar Petrovic"), age: 21, salary: 20_000 }
    }
}

pub fn structs() {
    let person1 = Person {
        name: String::from("Slobodan Dan"),
        age: 39,
        salary: 40_000,
    };

    println!(
        "{}, age {}, pays {} euro in taxes",
        person1.name,
        person1.age,
        person1.compute_taxes()
    );
    println!(
        "{}, age {}, pays {} euro in taxes",
        person1.name,
        person1.age,
        Person::compute_taxes(&person1)
    );

    let person2 = Person::new(String::new(), 12, 35).unwrap_or_default();

    println!(
        "{}, age {}, pays {} euro in taxes",
        person2.name,
        person2.age,
        Person::compute_taxes(&person2)
    );

    let person3 = Person {
        age: 50,
        name: String::from("Jelena Jelenovic"),
        ..person1
    };

    println!(
        "{}, age {}, pays {} euro in taxes",
        person3.name,
        person3.age,
        Person::compute_taxes(&person3)
    );

    let mut person4 = Person::default();
    person4.name = String::from("Nikola Nikolic");

    println!(
        "{}, age {}, pays {} euro in taxes",
        person4.name,
        person4.age,
        Person::compute_taxes(&person4)
    );

    let person5 = Person { name: String::from("Maja Majovic"), ..Default::default() };

    println!(
        "{}, age {}, pays {} euro in taxes",
        person5.name,
        person5.age,
        Person::compute_taxes(&person5)
    );
}

#[derive(Debug, Default)]
struct Numbers(i32, i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 >= self.1 { self.0 } else { self.1 }
    }

    fn lesser(&self) -> i32 {
        if self.0 < self.1 { self.0 } else { self.1 }
    }
}

pub fn tuple_structs() {
    let nums = Numbers(5, 10);
    println!("The point is at {} {}", nums.0, nums.1);
    println!("The lesser numbers is {} and greater is {}", nums.lesser(), nums.greater());
}
