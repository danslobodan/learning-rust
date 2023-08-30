use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn basic_example() {
    println!("The will be printed");
    println!("This will also be printed");
    println!("Conncurencty starts here");

    let t = thread::spawn(|| {
        println!("Hello 1 from the thread");
        println!("Hello 2 from the thread");
        println!("Hello 3 from the thread");
        println!("Hello 4 from the thread");
        println!("Hello 5 from the thread");
        println!("Hello 6 from the thread");
        println!("Hello 7 from the thread");
    });

    thread::sleep(Duration::from_millis(1));
    println!("Hello 1 from the main");
    println!("Hello 2 from the main");
    println!("Hello 3 from the main");
    println!("Hello 4 from the main");
    println!("Hello 5 from the main");
    t.join();
}

pub fn multiple_threads() {
    let mut thread_vec = vec![];

    for i in 0..10 {
        thread_vec.push(
            thread::spawn(move || {
                println!("Thread number {}", i);
            })
        );
    }

    for i in thread_vec {
        i.join();
    }
}

pub fn channels() {
    let (tx, rx) = mpsc::channel();

    let t = thread::spawn(move || {
        let val = String::from("I R baboon.");
        println!("Sending value from the thread");
        tx.send(val).unwrap();

        println!("This may execute after the statement in the main");
        // Value moved to the other thread - won't compile
        // println!("Val is {:?}", val);
    });

    // let received = rx.recv().unwrap();
    // println!("Received \"{}\"", received);

    // blocks main thread until the thread is finished
    // t.join();

    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("I have received {}", received_value);
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}

pub fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx.send(i).unwrap();
        }
    });

    for received_vals in rx {
        println!("I received the value of {}", received_vals);
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx.send(i).unwrap();
        }
    });

    let received_vals_vec = rx.iter().collect::<Vec<i32>>();
    println!("The received values are {:?}", received_vals_vec);

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let my_vec = vec![6, 7, 8, 9, 10];
        for i in my_vec {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received_vals in rx {
        println!("I received the value of {}", received_vals);
    }
}

fn timer(d: i32, tx: mpsc::Sender<i32>) {
    thread::spawn(move || {
        println!("{} send! ", d);
        tx.send(d).unwrap();
    });
}

pub fn threads_and_functions() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        timer(i, tx.clone());
    }
    drop(tx);

    for received_val in rx {
        println!("{} received val", received_val);
    }
}
