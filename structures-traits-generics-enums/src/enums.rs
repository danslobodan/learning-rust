fn examples() {
    let participant1 = Conveyance::Car(60);
    let participant2 = Conveyance::Train(120);
    let participant3 = Conveyance::Air(60);
    let participant4 = Conveyance::Walk;

    println!("The allowance for participant 1 is {}", participant1.travel_allowance());
    println!("The allowance for participant 2 is {}", participant2.travel_allowance());
    println!("The allowance for participant 3 is {}", participant3.travel_allowance());
    println!("The allowance for participant 4 is {}", participant4.travel_allowance());

    let val = vec![Value::Integer(12), Value::Float(15.5)];
    println!("The value of the integer is {:?} and the value of float is {:?}", val[0], val[1]);

    for i in val.iter() {
        match i {
            Value::Integer(num) => println!("The value is an integer {}", num),
            Value::Float(num) => println!("The value is a float {}", num),
        }
    }
}

enum Travel {
    Car = 15,
    Train = 20,
    Air = 30,
}

enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
    Walk,
}

impl Conveyance {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            Conveyance::Car(miles) => (*miles as f32) * 14.0 * 2.0,
            Conveyance::Train(miles) => (*miles as f32) * 18.0 * 2.0,
            Conveyance::Air(miles) => (*miles as f32) * 30.0 * 2.0,
            Conveyance::Walk => 0.0,
        };

        allowance
    }
}

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}
