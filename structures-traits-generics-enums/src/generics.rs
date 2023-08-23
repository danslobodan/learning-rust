fn examples() {
    println!("The square of 5 is {}", square(5));
    println!("The square of 5.5 is {}", square(5.5));

    let p1 = Point { x: 5, y: 5 };
    let p2 = Point { x: 5.5, y: 5 };
    let p3 = Point { x: 5, y: 5.5 };

    p1.print();
}

fn square<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

fn addition<T>(x: T) -> T where T: std::ops::Add<Output = T> + Copy {
    x + x
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> where T: std::fmt::Debug, U: std::fmt::Debug {
    fn print(&self) {
        println!("The value of the point coordinates are {:?}, {:?}", self.x, self.y);
    }
}
