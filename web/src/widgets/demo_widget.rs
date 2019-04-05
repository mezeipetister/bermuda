// Demo Widget

use crate::views::Widget;

pub struct Model {
    a: String,
}

impl Model {
    pub fn new(a: String) -> Self {
        Model { a }
    }
}

impl Widget for Model {
    fn render(&self) -> String {
        format!("<div>{}</div>", self.a)
    }
}
