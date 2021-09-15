use colored::*;

pub fn should_colour() -> bool {
    // Don't colour when running tests
    if cfg!(test) {
        false
    } else {
        !matches!(std::env::var("T_NO_COLOUR").as_deref(), Ok("true" | "1"))
    }
}

pub fn yellow(s: &str) -> String {
    s.yellow().to_string()
}

pub fn green(s: &str) -> String {
    s.green().to_string()
}

pub fn blue(s: &str) -> String {
    s.blue().to_string()
}

pub fn red(s: &str) -> String {
    s.red().to_string()
}

pub fn none(s: &str) -> String {
    s.to_string()
}
