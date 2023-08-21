fn main() {
    // Moving
    let n1 = String::from("abc");
    let n2 = n1;
    // println!("{}, {}", n1, n2); - won't compile

    // Borrowing
    let n3 = String::from("abc");
    let n4 = &n3;
    println!("String: Original {}, Borrowed {}", n3, n4);

    let vec1 = vec![1, 2, 3, 4];
    let vec2 = &vec1;
    println!("Vector: Original {:?}, Borrowed {:?}", vec1, vec2);

    // Cloning
    let vec3 = vec![1, 2, 3, 4];
    let vec4 = vec3.clone();
    println!("Vector: Original {:?}, Cloned {:?}", vec3, vec4);

    // Functions

    // Primitive (automatic) clone
    let num = 32;
    clone_and_print(num);
    println!("Main num {}", num);

    // Non-primitive move
    let moved_vec = vec![1, 2, 3];
    move_and_print(moved_vec);
    // println!("Main vec {}", vec); - won't compile

    // Non-primitive immutable borrow
    let borrowed_vec = vec![1, 2, 3];
    borrow_and_print(&borrowed_vec);
    println!("Main vec {:?}", borrowed_vec);

    // Non-primitive mutable borrow
    let mut mut_borrowed_vec = vec![1, 2, 3];
    borrow_and_change(&mut mut_borrowed_vec);
    println!("Main vec {:?}", mut_borrowed_vec);

    // Combine large data structures
    let large_data1 = String::from("This is the first long string");
    let large_data2 = String::from("This is the second long string");
    // The data from large_data1 and large_data2 will not be copied
    let huge_data = vec![&large_data1, &large_data2];

    println!("Huge data {:?}", huge_data);
}

fn one_mutable_reference() {
    // One mutable reference in a scope
    // Ensures no concurrent updates are made to the variable
    // Preventing data races

    let mut vec = vec![1, 2, 3];
    let ref1 = &mut vec;
    let ref2 = &mut vec;
    println!("{:?}, {:?}", ref1, ref2);
}

fn many_immutable_refs() {
    // Many immutable refs in a scope
    let mut vec = vec![1, 2, 3];
    let ref1 = &vec;
    let ref2 = &vec;
    println!("{:?}, {:?}", ref1, ref2);
}

fn either_mutable_or_immutable_refs() {
    // Either mutable or immutable references in a scope - not both
    let mut vec = vec![1, 2, 3];
    let ref1 = &vec;
    let ref2 = &vec;
    let ref3 = &mut vec;
    println!("{:?}, {:?}, {:?}", ref1, ref2, ref3);
}

fn immutable_and_mutable_together() {
    // Can mix mutable and immutable when the scope of one kind ends
    let mut vec = vec![1, 2, 3];
    let ref1 = &vec;
    let ref2 = &vec;
    println!("{:?}, {:?}", ref1, ref2);
    // Scope of ref1 and ref2 ended
    // Therefore they have finished their reading operations and it's safe to mutate the object
    // We can now decalre a mutable ref
    let ref3 = &mut vec;
    println!("{:?}", ref3);
}

fn cannot_change_data_before_read() {
    let mut vec = vec![1, 2, 3];
    // Valid modification - the immutable refs are still not in scope
    vec.push(4);
    let ref1 = &vec;
    let ref2 = &vec;
    // Invalid modification - cannot modify the data before read operations finish
    // vec.push(5); - won't compile
    println!("{:?}, {:?}", ref1, ref2);
    // Valid modification - the immutable refs went out of scope
    vec.push(6);
}

fn clone_and_print(mut num: i32) {
    num = 56;
    println!("Fun num: {}", num);
}

fn move_and_print(vec: Vec<i32>) {
    println!("Fun vec {:?}", vec);
}

fn borrow_and_print(vec: &Vec<i32>) {
    println!("Fun vec {:?}", vec);
}

fn borrow_and_change(vec: &mut Vec<i32>) {
    vec.push(4);
    println!("Fun mut vec {:?}", vec);
}
