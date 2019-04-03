use crate::catalog::Catalog;
use crate::controller::Widgets;

pub struct Html {
    title: String,
    body: String,
    css: Vec<String>,
    js: Vec<String>,
}

impl Html {
    pub fn new() -> Self {
        Html {
            title: String::new(),
            body: String::new(),
            css: Vec::new(),
            js: Vec::new(),
        }
    }

    // Set title in html!
    pub fn set_title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }

    // Add CSS to the html render!
    pub fn add_css(&mut self, css: String) -> &Self {
        self.css.push(css);
        self
    }

    // Add JS to the html render!
    pub fn add_js(&mut self, js: String) -> &Self {
        self.js.push(js);
        self
    }

    // Set template to render!
    pub fn set_body(&mut self, model: &Catalog, template: String) -> &Self {
        self
    }

    // Render HTML!
    pub fn render(&self) -> String {
        format!(
            "<!DOCTYPE html>
		<html>
		    <head>
			<title>{title}</title>
			<style>{css}</style>
		    </head>
		    <body>{body}{js}</body>
		</html>",
            title = self.title,
            body = self.body,
            css = self
                .css
                .iter()
                .map(|s: &String| s.chars())
                .flatten()
                .collect::<String>(),
            js = self
                .js
                .iter()
                .map(|s: &String| s.chars())
                .flatten()
                .collect::<String>()
        )
    }
}
