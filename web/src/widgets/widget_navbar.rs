// View Index

use crate::views::Widget;
use crate::widgets::*;

pub struct Model {
    css: String,
    js: String,
}

impl Model {
    pub fn new() -> Self {
         Model {
            css: String::new(),
            js: String::new(),
        }
    }

    pub fn add_css(&mut self, css: String) -> &Self {
        self.css.push_str(&css);
        self
    }
}

impl Widget for Model {
    fn render(&self) -> String {
        format!("
		<nav class=\"navbar navbar-expand-lg navbar-dark bg-secondary\">
		    <a class=\"navbar-brand\" href=\"#\">Bermuda</a>
		    <button class=\"navbar-toggler\" type=\"button\" data-toggle=\"collapse\" data-target=\"#navbarSupportedContent\" aria-controls=\"navbarSupportedContent\" aria-expanded=\"false\" aria-label=\"Toggle navigation\">
			<span class=\"navbar-toggler-icon\"></span>
		    </button>

		    <div class=\"collapse navbar-collapse\" id=\"navbarSupportedContent\">
			<ul class=\"navbar-nav mr-auto\">
			    <li class=\"nav-item active\">
				<a class=\"nav-link\" href=\"today\">Today<span class=\"sr-only\">(current)</span></a>
			    </li>
			    <li class=\"nav-item\">
				<a class=\"nav-link\" href=\"documents\">Documents</a>
			    </li>
			    <li class=\"nav-item\">
				<a class=\"nav-link\" href=\"folders\">Folders</a>
			    </li>
			</ul>
		    </div>
		</nav>"
        )
    }
}
