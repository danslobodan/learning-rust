use std::fs::{ *, self };
use std::io::{ BufRead, BufReader, Read, Write };
use std::path::{ Path, PathBuf };
use std::env;

pub fn example() {
    file_reading();
}

fn file_writing() -> std::io::Result<()> {
    let path_loc = r"C:\Users\danas\OneDrive\Documents\rust_example.txt";
    let path = Path::new(path_loc);
    let mut file = File::create(path)?;

    file.write(b"let's put this in the file")?;
    file.write("let's put htis in the file".as_bytes())?;

    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write("\n www.includehelp.com\n".as_bytes())?;

    let some_vec = vec![1, 2, 3, 4, 5, 6];
    let str_from_vec = some_vec
        .into_iter()
        .map(|a| format!("{} \n", a.to_string()))
        .collect::<String>();

    file.write(str_from_vec.as_bytes())?;

    Ok(())
}

fn file_reading() -> std::io::Result<()> {
    let path_loc = r"C:\Users\danas\OneDrive\Documents\rust_example.txt";
    let path = Path::new(path_loc);
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    println!("Contents: {}", contents);

    println!("Remove a file {:?}", fs::remove_file(path_loc));

    Ok(())
}

fn file_path() -> std::io::Result<()> {
    let path_loc = r"C:\Users\danas\OneDrive\Documents\rust_example.txt";
    let path = Path::new(path_loc);
    println!("Folder full path: {:?}", path.parent().unwrap());
    println!("Name: {:?}", path.file_stem().unwrap());
    println!("Extension: {:?}", path.extension().unwrap());

    let mut path = PathBuf::from(r"C:\");
    path.push(r"Rust");
    path.push(r"Examples");
    path.push(r"my_file");
    path.set_extension("txt");

    println!("Constructed path {:?}", path);

    let path = [r"C:\", "Rust", "Examples", "my_file.txt"].iter().collect::<PathBuf>();

    println!("Iterator path {:?}", path);

    let path = Path::new(r"C:\Slobodan");
    println!("Is {:?} a directory? {}", path, path.is_dir());
    println!("Is {:?} a file? {}", path, path.is_file());

    let data = path.metadata().unwrap();
    println!("type {:?}", data.file_type());
    println!("length {:?}", data.len());
    println!("Permission {:?}", data.permissions());
    println!("Modified {:?}", data.modified());
    println!("Created {:?}", data.created());

    Ok(())
}

fn directories() {
    let path = Path::new(r"C:\Slobodan");
    for file in path.read_dir().expect("read_dir call failed") {
        println!("{:?}", file.unwrap().path());
    }

    let curr_path = env::current_dir().expect("can't access current directory");
    println!("current path {:?}", curr_path);

    println!("Create a new directory {:?}", fs::create_dir(r"C:\Slobodan\newdir"));
    println!(
        "Create a new directory and subdirectories {:?}",
        fs::create_dir_all(r"C:\Slobodan\level1\level2")
    );

    println!("Remove a directory {:?}", fs::remove_dir(r"C:\Slobodan\newdir"));
    println!(
        "Remove a directory and it's contents {:?}",
        fs::remove_dir_all(r"C:\Slobodan\level1")
    );
}
