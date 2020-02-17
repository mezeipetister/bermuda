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

pub struct Folder {
    /**
     *
     */
    id: String,
    name: String,
    description: String,
    created_by: String,
    date_created: DateTime<Utc>,
    documents: Vec<Document>,
}

pub struct Document {
    /**
     * Unique ID    
     */
    id: String,
    /**
     * Reference ID,
     * use as you wish
     * e.g.: invoice/contract reference ID
     */
    reference: String,
    /**
     * Document title
     */
    title: String,
    /**
     * Short description
     */
    description: String,
    /**
     * Due date, e.g.: payment date for invoice,
     * or due date for contract
     */
    due_date: Date<Utc>,
    /**
     * ID for enclosed document PDF
     */
    file_id: String,
    /**
     * Created by user
     */
    created_by: String,
    /**
     * Date created
     */
    date_created: DateTime<Utc>,
}
