use std::env;
use std::fs;

pub fn load_file(abs_path: &str) {
    println!("Loading file: {}", abs_path);

    let contents = fs::read_to_string(abs_path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}