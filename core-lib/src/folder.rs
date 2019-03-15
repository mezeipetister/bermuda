// Folder object

use crate::doc::Doc;

pub enum FolderStatus {
    Open,
    Closed,
}

pub struct Folder {
    id: String,
    name: String,
    documents: Vec<Doc>,
    closed: FolderStatus,
}

impl Folder {
    pub fn new() -> Self {
        Folder {
            id: "ID".to_string(),
            name: "".to_string(),
            documents: Vec::new(),
            closed: FolderStatus::Open,
        }
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, new_name: String) -> Result<String, String> {
        self.name = new_name;
        Ok("Ok".to_string())
    }
    pub fn close(&mut self) -> Result<String, String> {
        self.closed = FolderStatus::Closed;
        Ok("Ok".to_string())
    }
    pub fn open(&mut self) -> Result<String, String> {
        self.closed = FolderStatus::Open;
        Ok("Ok".to_string())
    }
}
