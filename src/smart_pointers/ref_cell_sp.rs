use std::rc::Rc;
use std::cell::RefCell;

pub fn basic_examples() {
    // Can't have mutable and immutable refs
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;
    // println!("{} {}", x1, x2);

    let a = RefCell::new(10);
    {
        let b = a.borrow();
        let c = a.borrow();
    }

    // drop(b);
    // drop(c);

    // panics at runtime!
    let mut d = a.borrow_mut();

    drop(d);
    println!("{:?}", a);

    let a = RefCell::new(10);
    let mut b = a.borrow_mut();

    *b = 15;

    drop(b);

    // The value has been changed
    // even though "a" is declared as immutable
    println!("{:?}", a);

    let a = Rc::new(RefCell::new(String::from("Java")));
    let b = Rc::clone(&a);

    *b.borrow_mut() = String::from("C");
    println!("The new value is {:?}", a);
    println!("The new value is {:?}", b);

    *a.borrow_mut() = String::from("C++");
    println!("The new value is {:?}", a);
    println!("The new value is {:?}", b);
}
