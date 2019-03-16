// Catalog

use crate::doc::Doc;
use crate::files::*;
use crate::serde_yaml::*;
use std::fs::File;
//use crate::folder::*;

pub struct Catalog {
    documents: Vec<Doc>,
}

impl Catalog {
    /// Add document to catalog
    pub fn add_document_to_catalog(&mut self, doc: Doc) -> Result<String> {
        self.documents.push(doc);
        Ok("Ok".to_string())
    }
    /// Create a new document and
    /// put it to the catalog
    pub fn create_new_document(&mut self) -> Result<String> {
        let mut doc = Doc::new();
        doc.save().unwrap();
        self.add_document_to_catalog(doc).unwrap();
        Ok("Ok".to_string())
    }
    /// Get documents count stored under the catalog
    pub fn get_documents_number(&self) -> i32 {
        self.documents.len() as i32
    }
    pub fn get_documents(&self) -> &Vec<Doc> {
        &self.documents
    }
}

/// Init catalog!
/// Returns a catalog
pub fn init() -> Catalog {
    let mut _catalog = Catalog {
        documents: Vec::new(),
    };
    let path = get_home_path().unwrap().join(".bermuda").join("catalogs");
    create_folder_by_path(&path);
    for file_name in get_files_from_dir(&path) {
        let mut file = File::open(&path.join(file_name)).unwrap();
        let file_content = read_file_to_string(&mut file).unwrap();
        let _doc: Doc = serde_yaml::from_str(&file_content).unwrap();
        _catalog.add_document_to_catalog(_doc).unwrap();
    }
    _catalog
}
