#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: Membership,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Clone)]
enum Membership {
    new,
    loyal,
    rich,
}

impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name,
            username: None,
            membership: None,
            gender: None,
            country: None,
            age: None,
        }
    }
}

struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<Membership>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: Membership) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
        }
    }
}

impl Default for Membership {
    fn default() -> Self {
        Membership::new
    }
}

pub fn example() {
    let customer = Customer::new(String::from("Slobodan Dan"))
        .username(String::from("sloba"))
        .country(String::from("Serbia"))
        .age(39)
        .gender('m')
        .build();

    println!("{:?}", customer);
}
