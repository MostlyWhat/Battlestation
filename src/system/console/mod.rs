use crate::programs;
use crate::system::io::*;

// Console Function
pub fn main(username: &str) {
    clear();
    println!("MostlyWhat's Battlestation");
    println!("Console Version 0.0.1 (October 2022 Build)");
    println!("Copyright (C) 2022 MostlyWhat. All rights reserved.\n");
    println!("Type 'help' for a list of commands.\n");

    loop {
        let prefix = format!("{}@battlestation:~$ ", username).to_string();
        let command = input(&prefix);

        match command.as_str() {
            "clear" | "cls" => clear(),
            "help" => help(),
            "system" => system(),
            "exit" => exit(),
            _ => output(
                "Console",
                "Unknown Command, Type 'help' for a list of commands.",
            ),
        }
    }
}

// Available Commands & Programs
fn help() {
    let help_message = "Loading Help Message\n\
                                                    \n\
                                                    Available Commands:\n\
                                                    \n\
                                                    exit, quit - Exit the console.\n\
                                                    clear, cls - Clears the screen.\n\
                                                    help - Display this message.\n\
                                                    \n\
                                                    Available Programs:\n\
                                                    \n\
                                                    system - Display system information.";

    output("Help", help_message);
}

// System Information Function
fn system() {
    let system_info: &str = &programs::system::main().trim().to_string();

    output("System", system_info);
}

fn exit() {
    clear();
    println!("MostlyWhat's Battlestation");
    println!("Console Version 0.0.1 (October 2022 Build)");
    println!("Copyright (C) 2022 MostlyWhat. All rights reserved.\n");
    println!("Exiting...");
    println!("Please wait...\n");
    std::process::exit(0);
}
