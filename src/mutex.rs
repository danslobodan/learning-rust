use std::sync::{ Mutex, Arc, Barrier };
use std::thread;

pub fn basic_examples() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }

    println!("m = {:?}", m);

    let mut num = m.lock().unwrap();
    *num = 10;
    drop(num);

    let mut num1 = m.lock().unwrap();
    *num1 = 15;
    drop(num1);
}

pub fn muxetes_and_threads() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result {}", *counter.lock().unwrap());

    let mut threads = Vec::new();
    let name = Arc::new(MyString::new("Rust"));
    for i in 0..5 {
        let some_str = name.clone();
        let t = thread::spawn(move || {
            println!("Helo {} count {}", some_str.0, i);
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
    }
}

pub fn barrier() {
    let mut threads = Vec::new();
    let barrier = Arc::new(Barrier::new(5));

    for i in 0..10 {
        let barrier = barrier.clone();
        let t = thread::spawn(move || {
            println!("Before wait {}", i);
            barrier.wait();
            println!("After wait {}", i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}

struct MyString(String);
impl MyString {
    fn new(s: &str) -> MyString {
        MyString(s.to_string())
    }
}
