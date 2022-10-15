extern crate winres;
use std::env;

fn main() {
    // Get current app information
    let name = env::var("CARGO_PKG_NAME").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    // Set cargo environment variables for github actions
    env::set_var("PKG_NAME", name);
    env::set_var("PKG_VERSION", version);

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}
