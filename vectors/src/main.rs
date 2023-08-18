fn main() {
    let mut number_vec = vec![4, 5, 6, 7, 8];

    println!("First element of a vector {}", number_vec[0]);
    println!("Whole vector {:?}", number_vec);

    number_vec[4] = 2;
    println!("Modified vector {:?}", number_vec);

    let vector_with_same_elements = vec![0;10];
    println!("Vector of zeros length 10 {:?}", vector_with_same_elements);

    let mut string_vec = vec!["apple", "tomato", "grapes"];
    println!("Vector of strings {:?}", string_vec);

    let string_vec_same_elements = vec!["Unknown"; 6];
    println!("Vector of same strings length 6 {:?}", string_vec_same_elements);

    string_vec[0] = "Slobodan Dan";
    println!("Changed element of string vector {}", string_vec[0]);

    let char_vec = vec!['a', 'p', 'p', 'l', 'e'];
    println!("Char vector {:?}", char_vec[0]);

    let subset_vec = &&char_vec[0..3];
    println!("The subset values of the char vector are {:?}", subset_vec);

    println!("The length of the vector is {}", char_vec.len());

    let char_at_3 = char_vec.get(3);
    println!("Element at the index 3 is {:?}", char_at_3);

    let char_at_100 = char_vec.get(100);
    println!("Element at the index 100 is {:?}", char_at_100);

    let mut boom = vec!['b', 'o', 'o'];

    boom.push('m');
    println!("{:?}", boom);

    boom.pop();
    println!("{:?}", boom);

    let mut bomb = vec!['b', 'o', 'm', 'b'];
    bomb.remove(2);

    println!("{:?}", bomb);

    println!("The value of 5 exist in a number vector {}", number_vec.contains(&5));
    println!("The value of 10 exist in a number vector {}", number_vec.contains(&10));
}
