mod file_1;
mod file_2;
mod file_3;

use array_tool::vec::*;

struct Rectangle {
    length: i32,
    width: i32,
}

fn main() {
    let rect1 = Rectangle {
        length: 5,
        width: 10,
    };

    let area = file_1::rect_area(&rect1.length, &rect1.width);

    file_2::some_person();
    file_3::allowance();

    let vec1 = vec![1, 1, 3, 5, 6, 7];
    let vec2 = vec![1, 2, 3];

    let intersection = vec1.intersect(vec2.clone());
    println!("The intersection = {:?}", intersection);

    let intersection = vec1.union(vec2);
    println!("The union = {:?}", intersection);

    println!("Vec 1 three times {:?}", vec1.times(3));
}
