use std::ops::Drop;
use std::ops::Deref;

pub fn custom_smart_pointer_example() {
    let a = 50;
    let b = Box::new(a);
    println!("Box {}", 50 == a);

    // Deref trait allows Box to be deref-ed like a regular reference
    println!("Box {}", 50 == *b);
    // println!("{}", a == b);

    let sprt1 = MySmartPointer::new(a);
    let sprt2 = MySmartPointer::new(a + 3);
    let sprt2 = MySmartPointer::new(a + 6);
    println!("MySmartPointer {}", a == *sprt1); // *(sprt1.deref())

    drop(a);

    let sprt1 = MySmartPointer::new("Slobodan Dan");

    // This is possible due to Deref coercion
    // The compiler derefs the types until it finds the type it needs
    // &MySmartPointer -> &String -> &str
    my_fn(&sprt1);

    let some_vec = MySmartPointer::new(vec![1, 2, 3]);

    for z in &*some_vec {
        println!("The value is {}", z);
    }
}

struct MySmartPointer<T: std::fmt::Debug> {
    value: T,
}

impl<T: std::fmt::Debug> MySmartPointer<T> {
    fn new(value: T) -> Self {
        MySmartPointer { value }
    }
}

fn my_fn(str: &str) {
    println!("The string recieves from the main is {}", str);
}

impl<T: std::fmt::Debug> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: std::fmt::Debug> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping mysmartpointer object from memory {:?}", self.value);
    }
}
