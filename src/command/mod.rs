use rr::{rr_assert, rr_loge, rr_logi};
use crate::routines;

pub fn parse_command(args: Vec<String>) {
    println!("Parsing command: {:?}", args);
    // if args[0] contains "rr.exe"

    let mut look_up_index = 2;

    if (args[0].contains("rr.exe")) {
        look_up_index = 1;
    }

    match args[look_up_index].as_str() {
        "add" => {
            rr_assert!(args.len() >= 3, "Not enough arguments to add a service!");

            let path = args[2].clone();

            rr_assert!(path != "", "Service name cannot be empty!");

            routines::add_routine(&path);
        }
        _ => {
            rr_loge!("Unknown command");
        }
    }
}