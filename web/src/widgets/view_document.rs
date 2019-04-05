// View Index

use crate::doc::Doc;
use crate::views::Widget;
use crate::widgets::*;

pub struct Model {
    title: String,
    css: String,
    js: String,
    document: Doc,
}

impl Model {
    pub fn new(document: &Doc) -> Self {
        let mut model = Model {
            title: String::new(),
            css: String::new(),
            js: String::new(),
            document: Doc::new(),
        };

        // TODO: This is expensive! Solve it!
        model.document = document.clone();

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
			    <div class=\"row\">
				<div class=\"col-sm\">
				    <form method=\"POST\">
					<div class=\"form-group\">
					    <label for=\"title\">Documentum title</label>
					    <input type=\"text\" class=\"form-control\" id=\"title\" name=\"title\" aria-describedby=\"documentTitle\" placeholder=\"Enter document title\" value=\"{dtitle}\">
					    <!-- <small id=\"emailHelp\" class=\"form-text text-muted\">We'll never share your email with anyone else.</small> -->
					</div>
					<div class=\"form-group\">
					    <label for=\"description\">Description</label>
					    <input type=\"text\" class=\"form-control\" name=\"description\" id=\"description\" placeholder=\"Document description\" value=\"{ddescription}\">
					</div>
					<button type=\"submit\" class=\"btn btn-primary\">Save</button>
				    </form>
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
            dtitle = self.document.get_title(),
            ddescription = self.document.get_description(),
        )
    }
}
