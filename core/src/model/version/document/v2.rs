// Copyright (C) 2020 Peter Mezei
//
// This file is part of Bermuda.
//
// Bermuda is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Bermuda is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Bermuda.  If not, see <http://www.gnu.org/licenses/>.

use crate::folder::*;
use crate::model::version::document::v1;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Document {
    /**
     * Unique ID    
     */
    pub id: String,
    /**
     * Reference ID,
     * use as you wish
     * e.g.: invoice/contract reference ID
     */
    pub reference: String,
    /**
     * Folder reference
     */
    pub folder_id: String,
    /**
     * Document title
     */
    pub title: String,
    /**
     * Short description
     */
    pub description: String,
    /**
     * Due date, e.g.: payment date for invoice,
     * or due date for contract
     */
    pub due_date: Option<NaiveDate>,
    /**
     * ID for enclosed document PDF
     */
    pub file_id: Option<String>,
    /**
     * Created by user
     */
    pub created_by: String,
    /**
     * Date created
     */
    pub date_created: DateTime<Utc>,
    /**
     * Logical delete option
     * If its deleted its value false
     * otherwise its true
     */
    pub is_active: bool,
}

impl Document {
    pub fn new(
        created_by: String,
        reference: String,
        folder_id: String,
        title: String,
        description: String,
    ) -> Self {
        Document {
            id: generate_folder_id(),
            reference,
            folder_id,
            title,
            description,
            due_date: None,
            file_id: None,
            created_by,
            date_created: Utc::now(),
            is_active: true,
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Document::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        )
    }
}

impl VecPackMember for Document {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl From<v1::Document> for Document {
    fn from(from: v1::Document) -> Self {
        Document {
            id: from.id,
            reference: from.reference,
            folder_id: from.folder_id,
            title: from.title,
            description: from.description,
            due_date: if let Some(ddate) = from.due_date {
                Some(ddate.naive_local().date())
            } else {
                None
            },
            file_id: from.file_id,
            created_by: from.created_by,
            date_created: from.date_created,
            is_active: from.is_active,
        }
    }
}

impl TryFrom for Document {
    type TryFrom = v1::Document;
}
