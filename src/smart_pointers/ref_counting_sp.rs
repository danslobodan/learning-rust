enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{ Cons, Nil };
use std::rc::Rc;

pub fn basic_examples() {
    // Problem - can't have two references
    // let a = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("Counter after creating a = {}", Rc::strong_count(&a));

    {
        let b = Rc::new(Cons(3, Rc::clone(&a)));
        println!("Counter after creating a = {}", Rc::strong_count(&a));

        let c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("Counter after creating a = {}", Rc::strong_count(&a));
    }
    println!("Counter after creating a = {}", Rc::strong_count(&a));
}

pub fn string_example() {
    let s2 = make_rc();
    println!("Counter after function call {}", Rc::strong_count(&s2));
}

fn make_rc() -> Rc<String> {
    let s1 = Rc::new(String::from("Hello"));
    println!("Count when the pointer is created {}", Rc::strong_count(&s1));

    let s2 = s1.clone();

    s2
}
