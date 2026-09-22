use std::{
    fs::File,
    io::{self},
    path::Path,
};

fn main() {
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read file path");

    let file_path = file_path.trim();

    if Path::is_dir(Path::new(file_path)) {
        println!("failure");
        return;
    }

    match File::open(file_path) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}
