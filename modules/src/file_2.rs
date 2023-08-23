mod person {
    pub struct personal_info {
        pub age: u8,
        pub education: String,
    }

    impl personal_info {
        pub fn new(new_edu: &str) -> Self {
            Self {
                education: String::from(new_edu),
                age: 20,
            }
        }
    }
}

pub fn some_person() {
    let mut person1 = person::personal_info::new("Bachelors");
    person1.education = String::from("Masters");

    let person2 = person::personal_info {
        education: String::from("Master"),
        age: 42,
    };
}
