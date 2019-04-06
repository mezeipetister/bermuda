// Document store

use crate::files::*;
// use crate::serde_yaml::*;
// use std::fs::File; TODO: Remove it!
use std::fs::File;
use std::io;
use std::io::Write;
use std::path;
use std::path::Path;
// use std::path::Path; TODO: Remove it!

/// Doc struct
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Doc {
    id: String,
    title: String,
    description: String,
}

impl Doc {
    /// Get a new document
    pub fn new() -> Self {
        let id = crate::get_timestamp_as_micros();
        Doc {
            id: id,
            title: "".to_string(),
            description: "".to_string(),
        }
    }

    /// Save document
    /// 1. Encode the document to yaml
    /// 2. Save it as a yaml file
    /// TODO: Check error handling!
    pub fn save(&mut self) {
        save_document_to_file(
            &self.id,
            &serde_yaml::to_string(self).expect("Error writing string to file!"),
        )
        .expect("Error while saving document to file.");
    }

    // TODO: REFACT! JUST FOR TRIAl!
    pub fn add_attachment(&self) -> File {
        save_attachment(&self.id, &"INIT CONTENT".to_string())
    }

    /// Get id as string
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Get title as string
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Set title, returns Result
    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    /// Get description as string
    pub fn get_description(&self) -> &String {
        &self.description
    }

    /// Set desciption and return result
    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}

// TODO: REFACT! JUST FOR TRIAL!
fn save_attachment(document_id: &String, content: &String) -> File {
    let mut file = create_file_from_path(
        &get_home_path()
            .unwrap()
            .join(".bermuda") // TODO:! Move path to settings!
            .join("files") // TODO:! The same!
            .join(format!("{}.pdf", document_id)), // TODO: Manage extensions!
    )
    .unwrap();
    file.write(content.as_bytes());
    file
}

// Save document to file
// Helper function
fn save_document_to_file(file_id: &String, content: &String) -> io::Result<()> {
    write_string_to_file(
        &mut create_file_from_path(
            &get_home_path()
                .unwrap()
                .join(".bermuda") // TODO:! Move path to settings!
                .join("catalogs") // TODO:! The same!
                .join(format!("{}.yaml", file_id)),
        )
        .unwrap(),
        content,
    )
}
