/*
1. Write a program that calculates the set intersection and set union of two vectors containing some numbers. The program should have two separate functions: one for calculating the intersection and another for calculating the union.

The program will define two input vectors, vec_1 and vec_2, which contain unsigned 32-bit integers. For example:

let mut vec_1: Vec<u32> = vec![5, 4, 3, 6, 9];

let mut vec_2: Vec<u32> = vec![5, 8, 6, 4, 10, 15, 20, 21];

These vectors will be passed as arguments to the intersection and union functions to compute the respective results.
*/
fn soultion_1() {
    let vec_1: Vec<u32> = vec![5, 4, 3, 6, 9];
    let mut vec_2: Vec<u32> = vec![5, 8, 6, 4, 10, 15, 20, 21];

    let inter = intersection(&vec_1, &vec_2);
    println!("Intersection {:?}", inter);

    let union = union(&vec_1, &mut vec_2);
    println!("Intersection {:?}", union);
}

fn intersection(vec_1: &Vec<u32>, vec_2: &Vec<u32>) -> Vec<u32> {
    vec_1
        .clone()
        .into_iter()
        .filter(|x| vec_2.contains(x))
        .collect::<Vec<u32>>()
}

fn union(vec_1: &Vec<u32>, vec_2: &mut Vec<u32>) -> Vec<u32> {
    let mut result = vec_1.clone();
    result.append(vec_2);
    result
}

/*
2. For the code below defines a Person struct. In the main program, we created a vector that holds instances of Person and added several entries to it. 

Your task is to write a statement using iterators to collect all the ages of the different persons into a single vector.
 */
struct Person {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: i32,
}

fn solution_2() {
    let mut persons: Vec<Person> = Vec::new();

    persons.push(Person {
        first_name: "Nouman".to_string(),
        last_name: Some("Azam".to_string()),
        age: 1,
    });

    persons.push(Person {
        first_name: "Kamran".to_string(),
        last_name: Some("Khan".to_string()),
        age: 2,
    });

    persons.push(Person {
        first_name: "Rahul".to_string(),
        last_name: None,
        age: 6,
    });

    persons.push(Person {
        first_name: "Imran".to_string(),
        last_name: Some("Rehman".to_string()),
        age: 6,
    });

    // solution
    let ages = persons
        .iter()
        .map(|person| person.age)
        .collect::<Vec<i32>>();

    println!("Ages: {:?}", ages);
}

/*
3. You are developing a text processing utility in Rust. Your task is to implement a function called find_longest_word() that takes a string slice as input and returns a reference to the 
longest word in that string slice. You will need to handle cases where there are multiple words with the same length, in which case the function should return the first occurrence.

Write the find_longest_word() function, ensuring that the lifetime parameter is properly handled to ensure the validity of the returned reference.
 */
fn solution_3() {
    let str = "The quick brown fox jumps over the lazy dog.";
    let longest = longest_word(str);
    println!("Longest word is \"{}\"", longest);
}

fn longest_word(str: &str) -> &str {
    let words = str.split(' ').collect::<Vec<&str>>();
    let max_length = words
        .iter()
        .map(|&s| s.len())
        .max()
        .unwrap();

    let longest_word = words
        .iter()
        .find(|&&s| s.len() == max_length)
        .unwrap();

    longest_word
}
