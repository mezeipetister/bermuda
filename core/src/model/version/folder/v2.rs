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

use crate::model::version::folder::v1;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Folder {
    /**
     * Unique ID
     * Auto generated
     */
    pub id: String,
    /**
     * Folder name
     */
    pub name: String,
    /**
     * Folder description
     */
    pub description: String,
    /**
     * Created by userid
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

impl Default for Folder {
    fn default() -> Self {
        Folder::new("".to_string(), "".to_string(), "".to_string())
    }
}

impl VecPackMember for Folder {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl From<v1::Folder> for Folder {
    fn from(from: v1::Folder) -> Self {
        Folder {
            id: from.id,
            name: from.title,
            description: from.description,
            created_by: from.created_by,
            date_created: from.date_created,
            is_active: from.is_active,
        }
    }
}

impl TryFrom for Folder {
    type TryFrom = v1::Folder;
}
