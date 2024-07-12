#[macro_export]
macro_rules! rr_assert {
    ($cond:expr, $msg:expr) => {
        assert!($cond, $msg);
    };
}

// 1 - info
// 2 - warning
// 3 - error
// 4 - debug
// Set log level

#[macro_export]
macro_rules! rr_logi {
    ($msg:expr) => {
        // if LOG_LEVEL <= 1 {
        // }
        println!("\x1b[32m[INFO] {}\x1b[0m", $msg);
    };
}

#[macro_export]
macro_rules! rr_logw {
    ($msg:expr) => {
        // if LOG_LEVEL <= 2 {
        // }
        println!("\x1b[33m[WARNING] {}\x1b[0m", $msg);
    };
}

#[macro_export]
macro_rules! rr_loge {
    ($msg:expr) => {
        // if LOG_LEVEL <= 3 {
        // }
        println!("\x1b[31m[ERROR] {}\x1b[0m", $msg);
    };
}
