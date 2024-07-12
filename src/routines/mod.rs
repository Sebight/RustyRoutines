use crate::loader;

pub fn add_routine(path: &str) {
    println!("Routine added: {}", path);
    loader::load_file(path);
}

pub struct Routine {
    pub path: String,
    pub id: u32,
}