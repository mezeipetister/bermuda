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
    pub due_date: Option<DateTime<Utc>>,
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

impl TryFrom for Document {
    type TryFrom = Document;
}
