extern crate dirs;

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;
use std::path::Path;

/// Init process
/// Only run once at startup
pub fn init() {
    // Create ~/-bermuda path to home dir
    let path = get_home_path()
        .expect("Expected home dir determination error!")
        .join(".bermuda");
    // Create folder
    create_folder_by_path(&path);

    get_files_from_dir(&get_home_path().expect("Ooo").join("Downloads"));

    // let data_path = format!("{}/.bermuda/data/", get_home_path());
    // // Create ~/.bermuda/data folder if does not exist
    // create_folder_by_path(&data_path);
    // let file_path = File::create("~/Desktop/demo.txt");
    // write_string_to_file(&mut file_path.unwrap(), b"Hello World!")
    //     .expect("Ooo error during file write.");
    // let c = read_file_to_string(&data_path).expect("Error occured during file read.");
    // println!("File content: {}", c);
}

/// Get Path by String
pub fn get_path_by_string(path: &String) -> &Path {
    Path::new(path)
}

/// Create folder by path
/// Give a path and create a folder with sub folders
/// If the folder exists then it wont replace it!
/// TODO: use result! Refact!
pub fn create_folder_by_path(path: &Path) {
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
pub fn get_home_path() -> Option<path::PathBuf> {
    dirs::home_dir()
}

pub fn get_files_from_dir(dir: &Path) -> Vec<String> {
    if dir.is_dir() {
        let names = fs::read_dir(dir)
            .expect("Error during reading folder..")
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                })
            })
            .collect::<Vec<String>>();
        return names;
    }
    Vec::new()
}

/// Read file to string
pub fn read_file_to_string(file: &mut File) -> Result<String, io::Error> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Write string to file
pub fn write_string_to_file(path: &mut File, content: &[u8]) -> std::io::Result<()> {
    // fs::write(path, content)
    path.write(content)?;
    Ok(())
}

/// Create file by path
pub fn create_file(file_path: &Path) -> io::Result<File> {
    File::create(file_path)
}
