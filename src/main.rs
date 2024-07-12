use std::env;

mod command;
mod routines;
mod loader;

pub struct AppState {
    pub routines: Vec<String>,
}

fn main() {
    let app_state = AppState {
        routines: vec![]
    };
    
    let args: Vec<String> = env::args().collect();
    command::parse_command(args);
}
