#![feature(proc_macro_hygiene, decl_macro, plugin)]

pub mod views;
pub mod widgets;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

extern crate core_lib;
use core_lib::catalog;
use core_lib::doc;
use rocket::request::Form;
use rocket::response::content;
use rocket::response::Redirect;
use rocket::State;
use std::sync::Mutex;

use crate::views::Widget;
use crate::widgets::*;

#[get("/")]
fn index(_c: State<View>) -> content::Html<String> {
    let mut a = view_index::Model::new();
    a.set_title("Bermuda".to_string());
    content::Html(a.render())
    // content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
}

#[get("/about")]
fn about(_c: State<View>) -> content::Html<String> {
    let mut a = view_about::Model::new();
    a.set_title("Bermuda".to_string());
    content::Html(a.render())
    // content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
}

#[get("/today")]
fn today(_c: State<View>) -> content::Html<String> {
    let mut a = view_index::Model::new();
    a.set_title("Bermuda".to_string());
    content::Html(a.render())
    // content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
}

#[get("/documents")]
fn documents(_c: State<View>) -> content::Html<String> {
    let mut a = view_documents::Model::new(&_c.inner().models.lock().unwrap());
    a.set_title("Bermuda".to_string());
    content::Html(a.render())
    // content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
}

#[get("/document/<id>")]
fn document(_c: State<View>, id: String) -> content::Html<String> {
    let mut a =
        view_document::Model::new(&_c.inner().models.lock().unwrap().get_document_by_id(id));
    a.set_title("Bermuda".to_string());
    content::Html(a.render())
    // content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
}

#[post("/document/<id>", data = "<data>")]
fn document_save(_c: State<View>, id: String, data: Form<_Document>) -> Redirect {
    let mut documents = _c.inner().models.lock().unwrap();
    let d = documents.get_document_by_id(id.clone());
    d.set_title(data.title.clone());
    d.set_description(data.description.clone());
    d.save();
    Redirect::to(format!("/document/{}", id))
}

#[get("/folders")]
fn folders(_c: State<View>) -> content::Html<String> {
    let mut a = view_index::Model::new();
    a.set_title("Bermuda".to_string());
    content::Html(a.render())
    // content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
}

#[get("/new")]
fn form(_c: State<View>) -> content::Html<String> {
    let mut a = view_new_document::Model::new();
    a.set_title("Bermuda".to_string());
    content::Html(a.render())
    // content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
}

#[derive(FromForm)]
struct _Document {
    title: String,
    description: String,
}

#[post("/new", data = "<data>")]
fn create(_c: State<View>, data: Form<_Document>) -> Redirect {
    let mut d = doc::Doc::new();
    let id = d.get_id();
    d.set_title(data.title.clone());
    d.set_description(data.description.clone());
    d.save();
    _c.inner().models.lock().unwrap().add_document_to_catalog(d);
    Redirect::to(format!("/document/{}", id))
}

struct View {
    models: Mutex<catalog::Catalog>,
}

fn main() {
    let catalog = catalog::init();

    let c = View {
        models: Mutex::new(catalog),
    };

    rocket::ignite()
        .manage(c)
        .mount(
            "/",
            routes![
                index,
                today,
                about,
                document,
                document_save,
                documents,
                folders
            ],
        )
        .mount("/document", routes![form, create])
        .launch();
}
