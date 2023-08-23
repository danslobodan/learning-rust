use std::collections::HashMap;

fn main() {
    let student1 = Student {
        id: 1,
        name: String::from("Jelena"),
        grade: String::from("A"),
    };

    let student2 = Student {
        id: 2,
        name: String::from("Petar"),
        grade: String::from("B"),
    };

    let student3 = Student {
        id: 2,
        name: String::from("Dusan"),
        grade: String::from("B"),
    };

    let mut manager = StudentManager::new();
    manager.add_student(student1);
    manager.add_student(student2);

    match manager.add_student(student3) {
        Err(msg) => println!("Error: {}", msg),
        Ok(_) => println!("Student added"),
    }

    match manager.get_student(2) {
        Some(student) =>
            println!(
                "Found student {} with id {} and grade {}",
                student.name,
                student.id,
                student.grade
            ),
        None => println!("There is no student with given id"),
    }
}

struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    manager: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager { manager: HashMap::new() }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.manager.contains_key(&student.id) {
            Err(String::from("Student ID already exists."))
        } else {
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        self.manager.get(&id)
    }
}
