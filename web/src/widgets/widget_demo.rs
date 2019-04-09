// Demo Widget

use crate::widgets::Widget;

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

pub fn new() -> Model {
    Model { a: "Peti".to_string() }
}
