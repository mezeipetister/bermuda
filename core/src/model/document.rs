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

pub use crate::model::version::document::v2::Document;
use chrono::prelude::*;

impl Document {
    pub fn get_folder(&self) -> &str {
        &self.folder_id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_reference(&mut self, reference: String) {
        self.reference = reference;
    }
    pub fn get_reference(&self) -> &str {
        &self.reference
    }
    pub fn set_folder(&mut self, folder_id: String) {
        self.folder_id = folder_id;
    }
    pub fn set_file(&mut self, file_id: Option<String>) {
        self.file_id = file_id;
    }
    pub fn set_due_date(&mut self, due_date: Option<NaiveDate>) {
        self.due_date = due_date;
    }
    pub fn get_due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn remove(&mut self) {
        self.is_active = false;
    }
    pub fn restore(&mut self) {
        self.is_active = true;
    }
}
