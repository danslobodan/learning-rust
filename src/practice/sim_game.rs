pub fn run() {
    let dog1 = Dog::new(1, "Dog 1".to_string(), 2);
    let dog2 = Dog::new(2, "Dog 2".to_string(), 2);
    let dog3 = Dog::new(3, "Dog 3".to_string(), 4);

    println!("Can {} be traded with {}: {}", &dog1.name, &dog2.name, dog1.trade(&dog2));
    println!("Can {} be traded with {}: {}", &dog3.name, &dog2.name, dog3.trade(&dog2));
}

#[derive(Debug)]
struct Dog {
    id: u32,
    name: String,
    age: u32,
}

impl Dog {
    fn new(id: u32, name: String, age: u32) -> Self {
        Self { id, name, age }
    }
}

trait Trade {
    type Pet;

    fn trade(&self, other: &Self::Pet) -> bool;
}

impl Trade for Dog {
    type Pet = Dog;

    fn trade(&self, other: &Self::Pet) -> bool {
        if self.id > other.id { true } else { false }
    }
}

trait Playable: Trade {}

impl Playable for Dog {}
