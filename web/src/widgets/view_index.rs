// View Index

use crate::widgets::*;

pub struct Model {
    title: String,
    css: String,
    js: String,
}

impl Model {
    pub fn new() -> Self {
        let mut model = Model {
            title: String::new(),
            css: String::new(),
            js: String::new(),
        };
        model.add_css(format!(" html, body {{margin:0; padding:0;}}"));
        model.add_css(include_str!("../../assets/bootstrap.min.css").to_string());
        model.add_js(include_str!("../../assets/jquery-3.3.1.slim.min.js").to_string());
        model.add_js(include_str!("../../assets/popper.min.js").to_string());
        model.add_js(include_str!("../../assets/bootstrap.min.js").to_string());
        model
    }

    pub fn add_css(&mut self, css: String) -> &Self {
        self.css.push_str(&css);
        self
    }

    pub fn add_js(&mut self, js: String) -> &Self {
        self.js.push_str(&js);
        self
    }

    pub fn set_title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }
}

impl Widget for Model {
    fn render(&self) -> String {
        format!(
            "<!DOCTYPE html>
		<html>
		    <head>
			<title>{title}</title>
			<style>{css}</style>
		    </head>
		    <body>
			<div class=\"container\">
			    {navbar}
			    <div class=\"row\"></div>
			</div>
			<script>{js}</script>
		    </body>
		</html>",
            title = self.title,
            css = self.css,
            js = self.js,
            navbar = widget_navbar::Model::new().render(),
        )
    }
}
