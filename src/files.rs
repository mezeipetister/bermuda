extern crate dirs;

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

/// Init process
/// Only run once at startup
fn init() {
    // let data_path = format!("{}/.bermuda/data/", get_home_path());
    // // Create ~/.bermuda/data folder if does not exist
    // create_folder_by_path(&data_path);
    // let file_path = File::create("~/Desktop/demo.txt");
    // write_string_to_file(&mut file_path.unwrap(), b"Hello World!")
    //     .expect("Ooo error during file write.");
    // let c = read_file_to_string(&data_path).expect("Error occured during file read.");
    // println!("File content: {}", c);
}

/// Create folder by path
/// Give a path and create a folder with sub folders
/// If the folder exists then it wont replace it!
/// TODO: use result! Refact!
fn create_folder_by_path(path: &String) {
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

fn get_files_from_dir(dir: &String) {
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }
}

/// Read file to string
fn read_file_to_string(file: &mut File) -> Result<String, io::Error> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Write string to file
fn write_string_to_file(path: &mut File, content: &[u8]) -> std::io::Result<()> {
    // fs::write(path, content)
    path.write(content)?;
    Ok(())
}

fn create_file(file_path: &String) -> io::Result<File> {
    File::create(file_path)
}
