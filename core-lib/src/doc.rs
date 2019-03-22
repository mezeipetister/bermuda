// Document store

use crate::files::*;
use crate::serde_yaml::*;
// use std::fs::File; TODO: Remove it!
use std::io;
// use std::path::Path; TODO: Remove it!

/// Doc struct
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

    /// Get id as string
    pub fn get_id(&self) -> &String {
        &self.id
    }

    /// Get title as string
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Set title, returns Result
    pub fn set_title(&mut self, new_title: String) -> Result<()> {
        self.title = new_title;
        Ok(())
    }
    
    /// Get description as string
    pub fn get_description(&self) -> &String {
        &self.description
    }

    /// Set desciption and return result
    pub fn set_description(&mut self, new_description: String) -> Result<()> {
        self.description = new_description;
        Ok(())
    }
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
