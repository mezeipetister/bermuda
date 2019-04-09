// View Index

use crate::catalog::Catalog;
use crate::widgets::*;

use core_lib::doc::Doc;

pub struct Model {
    title: String,
    css: String,
    js: String,
    documents: Vec<Doc>,
}

impl Model {
    pub fn new(catalog: &Catalog) -> Self {
        let mut model = Model {
            title: String::new(),
            css: String::new(),
            js: String::new(),
            documents: Vec::new(),
        };

        // TODO: This is expensive! Solve it!
        model.documents = catalog.get_documents().clone();

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
        let mut docs_list = String::new();

        for doc in &self.documents {
            let title = doc.get_title().clone();
            let id = doc.get_id().clone();
            docs_list.push_str(&format!(
                "<li><a href=\"/document/{id}\">{doc_title}</a></li>",
                id = id,
                doc_title = title
            ));
        }

        format!(
            "<!DOCTYPE html>
		<html>
		    <head>
			<title>{title}</title>
			<style>{css}</style>
		    </head>
		    <body>
			    {navbar}
			<div class=\"container-fluid\">
			    <div class=\"row\">
				<div class=\"col-sm\">
				    <ul>{docs_list}</ul>
				</div>
			    </div>
			</div>
			<script>{js}</script>
		    </body>
		</html>",
            title = self.title,
            css = self.css,
            js = self.js,
            navbar = widget_navbar::Model::new().render(),
            docs_list = docs_list,
        )
    }
}
