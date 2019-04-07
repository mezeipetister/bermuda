// Catalog

use crate::doc::Doc;
use crate::files::*;
// use crate::serde_yaml::*; TODO: Why?
use std::fs::File;
//use crate::folder::*;

// Catalog struct
// Contains documents as vector
pub struct Catalog {
    documents: Vec<Doc>,
}

impl Catalog {
    /// Add document to catalog
    pub fn add_document_to_catalog(&mut self, doc: Doc) {
        self.documents.push(doc);
    }

    /// Get documents count
    pub fn get_documents_number(&self) -> i32 {
        self.documents.len() as i32
    }

    /// Hello Bello
    pub fn get_documents(&self) -> &Vec<Doc> {
        &self.documents
    }

    /// Get Document by id
    /// Returns a result
    /// TODO: Check error handling!
    pub fn get_document_by_id(&mut self, id: String) -> Result<&mut Doc, ()> {
        let index = self
            .documents
            .iter()
            .position(|doc| doc.get_id().clone() == id);
        match index {
            Some(index) => Ok(&mut self.documents[index]),
            None => Err(()),
        }
    }
}

/// Init catalog!
/// Returns a catalog
/// Should init just once at start!
pub fn init() -> Catalog {
    // Init catalog to return at the end
    let mut _catalog = Catalog {
        documents: Vec::new(),
    };

    // TODO: set path in config? Instead of decalre here.
    let path = get_home_path().unwrap().join(".bermuda").join("catalogs");

    // Check and create home directory
    create_folder_by_path(&path).expect("Error while creating folder!");

    for file_name in get_files_from_dir(&path) {
        // Get file from a given file name
        let mut file =
            File::open(&path.join(file_name)).expect("Error while get file from file name");

        // Read the given file to string;
        let _doc: Doc = serde_yaml::from_str(
            &read_file_to_string(&mut file).expect("Error while reading file to string"),
        )
        .expect("Error while decoding yaml to string");

        // Add read document to the catalog.
        _catalog.add_document_to_catalog(_doc);
    }

    // Return catalog
    _catalog
}
