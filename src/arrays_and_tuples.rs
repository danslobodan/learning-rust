fn examples() {
    tuples();
    arrays();
    distance();
}

fn tuples() {
    let p1: (f64, f64) = (3.5, 4.0);
    let p2: (f64, f64) = (2.7, 5.8);

    println!("X diff {}, Y diff {}", (p1.0 - p2.0).abs(), (p1.1 - p2.1).abs());
}

fn arrays() {
    let p1: [f64; 2] = [3.5, 4.0];
    let p2: [f64; 2] = [2.7, 5.8];

    println!("X diff {}, Y diff {}", (p1[0] - p2[0]).abs(), (p1[1] - p2[1]).abs());
}

fn distance() {
    let p1: (f64, f64) = (4.0, 3.0);
    let p2: (f64, f64) = (5.0, 4.5);

    let distance = ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).sqrt();

    println!("The distance is {}", distance);
}
