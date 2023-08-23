use std::vec;

fn main() {
    mapping();
}

fn simple_example() {
    let some_vec = vec![1, 2, 3];
    let mut iter = some_vec.iter();

    println!("The iterator {:?}", iter);
    println!("Iterator value {:?}", iter.next());
    println!("Iterator value {:?}", iter.next());
    println!("Iterator value {:?}", iter.next());
    println!("Iterator value {:?}", iter.next());
    println!("Iterator value {:?}", iter.next());
}

fn iterator_functions() {
    let vec_1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let gt_zero = vec_1.iter().any(|&x| x > 0);
    println!("Are there values greater than zero? {}", gt_zero);

    let all_gt_zero = vec_1.iter().all(|&x| x > 0);
    println!("Are all values greater than zero? {}", all_gt_zero);

    let first_gt_zero = vec_1.iter().find(|&&x| x > 0);
    println!("The value of function find is {}", first_gt_zero.unwrap());

    let at_index = vec_1.iter().position(|&x| x > 4);
    println!("The position of the first element greater than 4 is {}", at_index.unwrap());

    let rev_index = vec_1.iter().rposition(|&x| x < 6);
    println!(
        "The position of the first element from the end, that is greater than 4 is {}",
        rev_index.unwrap()
    );

    let min_value = vec_1.iter().min();
    println!("The min value is {}", min_value.unwrap());

    let max_value = vec_1.iter().max();
    println!("The max value is {}", max_value.unwrap());

    let mut rev_iter = vec_1.iter().rev();
    println!("The reverse iterator {:?}", rev_iter);
    println!("Rev value {:?}", rev_iter.next());
    println!("Rev value {:?}", rev_iter.next());
    println!("Rev value {:?}", rev_iter.next());
}

fn filtering() {
    let vec_1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

    // vector of references
    let filtered = vec_1
        .iter()
        .filter(|&x| *x > 2 && *x < 6)
        .collect::<Vec<&u32>>();
    println!("Subset {:?}", filtered);

    println!("{:?}", vec_1);

    // vector of values
    // uses into_iter
    let filtered = vec_1
        .into_iter()
        .filter(|&x| x > 2 && x < 6)
        .collect::<Vec<u32>>();
    println!("Subset {:?}", filtered);

    // vec_1 is now borrowed so the code bellow will not compile
    // println!("{:?}", vec_1);

    // cloned vector
    let vec_1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let filtered = vec_1
        .clone()
        .into_iter()
        .filter(|&x| x > 2 && x < 6)
        .collect::<Vec<u32>>();
    println!("Subset {:?}", filtered);

    // vec_1 is no longer borrowed - it is cloned
    println!("{:?}", vec_1);
}

fn mapping() {
    let vec_1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let squared = vec_1
        .iter()
        .map(|&x| x * x)
        .collect::<Vec<u32>>();

    println!("Squared {:?}", squared);
}
