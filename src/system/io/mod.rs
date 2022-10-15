extern crate rpassword;

use std::io;
use std::io::Write;

// Normal Input Function
pub fn input(text: &str) -> String {
    print!("{}", text);
    std::io::stdout()
        .flush()
        .ok()
        .expect("Failed to flush stdout");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    return user_input.trim().to_string();
}

// Password Input Function
pub fn password_input(text: &str) -> String {
    let user_input = rpassword::prompt_password(text).unwrap();
    return user_input.trim().to_string();
}

// Normal Output Function
pub fn output(program: &str, text: &str) {
    println!("\n[ {} ] {}\n", program, text);
}

// Clear Screen Function
pub fn clear() {
    clearscreen::clear().unwrap();
}
