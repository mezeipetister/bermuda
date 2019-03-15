// Catalog

use crate::doc::Doc;
use crate::files::*;
//use crate::folder::*;

pub struct Catalog {
    documents: Vec<Doc>,
}

impl Catalog {
    pub fn add_to_catalog(&mut self, doc: Doc) -> Result<String, String> {
        self.documents.push(doc);
        Ok("Ok".to_string())
    }
}

/// Init catalog!
/// Returns a catalog
pub fn init() -> Catalog {
    create_folder_by_path(&get_home_path().unwrap().join(".bermuda").join("catalogs"));
    Catalog {
        documents: Vec::new(),
    }
}
