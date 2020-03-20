use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Folder0 {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub date_created: DateTime<Utc>,
    pub is_active: bool,
}

impl VecPackMember for Folder0 {
    fn get_id(&self) -> &str {
        &self.id
    }
}
