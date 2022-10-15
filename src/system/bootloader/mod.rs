use crate::system::console;
use crate::system::io::*;

pub fn loading() {
    clear();
    println!("MostlyWhat's Battlestation");
    println!("Loading...");
    println!("Please wait...\n");
}

pub fn menu() {
    clear();
}

pub fn decryption() -> bool {
    
}

pub fn authentication() -> bool {
    clear();

    output(
        "Authentication",
        "Please enter your username and password.\n",
    );
    // Collect Username
    let username = input("[ Authentication ] Username: ");

    // Collect Password
    let password = password_input("[ Authentication ] Password: ");

    // Check if Username and Password are correct
    if username == "mostlywhat" && password == "admin" {
        return true;
    } else {
        return false;
    }
}

pub fn console(username: &str) {
    console::main(username);
}
