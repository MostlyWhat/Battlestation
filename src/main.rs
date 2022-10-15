mod programs;
mod system;

fn main() {
    // Loading Screen
    system::bootloader::loading();

    // Authentication
    let correct_login = system::bootloader::authentication();

    // Check if Login is correct
    if correct_login {
        // Run Console
        system::bootloader::console("mostlywhat");
    } else {
        // Display Error
        system::io::output("Authentication", "Incorrect username or password.");
    }
}
