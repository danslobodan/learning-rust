use std::thread;

pub fn example() {
    let mut vec = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("I am the first thread in the scope.");
            println!("{:?}", vec);
        });

        some_scope.spawn(|| {
            println!("I am the second thread in the scope.");
            x += 45;
            println!("{}", x);
        });
    });

    println!("The threads are now complete");
    vec.push(5);
    println!("x: {:?} and vec: {:?}", x, vec);
}
