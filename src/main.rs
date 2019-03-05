extern crate dirs;

use std::fs;
use std::path::Path;
use std::ffi::OsString;

fn main() {
    init();
    get_files_from_dir(format!("{}/Documents",get_home_path()));
}

/// Init process
/// Only run once at startup
fn init() {
    // Create ~/.bermuda/data folder if does not exist
    create_folder_by_path(format!("{}/.bermuda/data", get_home_path()));
}

/// Create folder by path
/// Give a path and create a folder with sub folders
/// If the folder exists then it wont replace it!
/// TODO: use result! Refact!
fn create_folder_by_path(path: String) {
    match fs::create_dir_all(path) {
        Err(why) => panic!("OOoo Error occured! Why?: {}", why),
        Ok(()) => println!("Ok!"),
    }
}

/// Get home path
/// Work on windows, mac, linux
/// # Example
/// ```rust
/// println!("Home path is: {}",get_home_path())
/// ```
fn get_home_path() -> String {
    match dirs::home_dir() {
        Some(path) => format!("{}", path.display()),
        None => panic!("Error occured while home directory determined..."),
    }
}

fn get_files_from_dir(dir: String) {
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }
}
