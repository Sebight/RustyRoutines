pub fn add_routine(path: &str) {
    println!("Routine added: {}", path);
}

pub struct Routine {
    pub path: String,
    pub id: u32,
}