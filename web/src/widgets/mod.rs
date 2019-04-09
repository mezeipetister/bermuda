// Widgets

pub trait Widget {
    fn render(&self) -> String;
}

// Views;
pub mod view_index;
pub mod view_about;
pub mod view_document;
pub mod view_documents;
pub mod view_new_document;

// Widgets
pub mod widget_demo;
pub mod widget_navbar;
