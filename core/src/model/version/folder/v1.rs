use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Folder {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub date_created: DateTime<Utc>,
    pub is_active: bool,
}

impl Default for Folder {
    fn default() -> Self {
        Folder {
            id: String::default(),
            title: String::default(),
            description: String::default(),
            created_by: String::default(),
            date_created: Utc::now(),
            is_active: true,
        }
    }
}

impl VecPackMember for Folder {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl TryFrom for Folder {
    type TryFrom = Folder;
}
