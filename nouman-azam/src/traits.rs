pub fn examples() {
    let speed = Kmh { value: 90 };
    let distance = speed.distance(3);

    println!("At {:?}, you will travel {:?} in 3 hours", speed, distance);

    let speed = Mph { value: 90 };
    let distance = speed.distance(3);

    println!("At {:?}, you will travel {:?} in 3 hours", speed, distance);
}

struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
}

struct Student {
    name: String,
    age: u8,
    sex: char,
    country: String,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name, self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name, self.age, self.sex)
    }

    fn country_info(&self) -> &str {
        &self.country
    }
}

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}

impl Double for i64 {
    fn double(&self) -> Self {
        self * 2
    }
}

fn quadruple<T: Double>(x: T) -> T {
    x.double().double()
}

#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}

#[derive(Debug)]
struct Miles {
    value: u32,
}

trait Distance {
    type Distance;

    fn distance(&self, hours: u32) -> Self::Distance;
}

impl Distance for Kmh {
    type Distance = Km;

    fn distance(&self, hours: u32) -> Self::Distance {
        Km { value: self.value * hours }
    }
}

impl Distance for Mph {
    type Distance = Miles;

    fn distance(&self, hours: u32) -> Self::Distance {
        Miles { value: self.value * hours }
    }
}
