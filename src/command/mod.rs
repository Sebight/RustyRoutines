use rr::{rr_assert, rr_loge, rr_logi};
use crate::routines;

pub fn parse_command(args: Vec<String>) {
    // We walk through the args vector and match the first argument to a command
    match args[2].as_str() {
        "add" => {
            rr_assert!(args.len() >= 4, "Not enough arguments to add a service!");

            let routine_name = args[3].clone();

            rr_assert!(routine_name != "", "Service name cannot be empty!");

            routines::add_routine(&routine_name);
        }
        _ => {
            rr_loge!("Unknown command");
        }
    }
}

