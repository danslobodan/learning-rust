use std::mem::{ self, swap };

#[derive(Debug)]
enum Customer {
    new {
        name: String,
    },
    loyal {
        name: String,
    },
    rich {
        name: String,
    },
}

fn promote(user: &mut Customer) {
    *user = match user {
        Customer::new { name } => Customer::loyal { name: mem::replace(name, String::new()) },
        Customer::loyal { name } => Customer::rich { name: mem::take(name) },
        Customer::rich { name } => {
            return;
        }
    };
}

pub fn examples() {
    let mut customer1 = Customer::new { name: "michael".to_string() };
    println!("Customer 1 {:?}", customer1);

    promote(&mut customer1);
    println!("Customer 1 {:?}", customer1);

    promote(&mut customer1);
    println!("Customer 1 {:?}", customer1);

    promote(&mut customer1);
    println!("Customer 1 {:?}", customer1);

    let mut s1 = "Slobodan".to_string();
    let mut s2 = "Dan".to_string();

    println!("{} {}", s1, s2);

    swap(&mut s1, &mut s2);

    println!("{} {}", s1, s2);
}
