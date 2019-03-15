// Document store

use crate::files::*;
use crate::serde_yaml::*;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Doc {
    id: String,
    title: String,
    description: String,
}

impl Doc {
    pub fn new() -> Self {
        let id = "ID".to_string();
        _create_file(&id, &"".to_string());
        Doc {
            id: id,
            title: "".to_string(),
            description: "".to_string(),
        }
    }
    pub fn save(&mut self) -> Result<String> {
        _create_file(
            &self.id,
            &serde_yaml::to_string(self).expect("Error writing string to file!"),
        );
        Ok("Ok".to_string())
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn set_title(&mut self, new_title: String) -> Result<String> {
        self.title = new_title;
        Ok(String::from("Ok!"))
    }
    pub fn get_description(&self) -> &String {
        &self.description
    }
    pub fn set_description(&mut self, new_description: String) -> Result<String> {
        self.description = new_description;
        Ok(String::from("Ok!"))
    }
}

fn _create_file(file_id: &String, content: &String) -> io::Result<()> {
    write_string_to_file(
        &mut create_file(
            &get_home_path()
                .unwrap()
                .join(".bermuda")
                .join("catalogs")
                .join(format!("{}.yaml", file_id)),
        )
        .unwrap(),
        content.as_bytes(),
    )
}
