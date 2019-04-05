#![feature(proc_macro_hygiene, decl_macro, plugin)]

pub mod views;
pub mod widgets;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

extern crate core_lib;
use core_lib::catalog;
use rocket::response::content;
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

#[get("/folders")]
fn folders(_c: State<View>) -> content::Html<String> {
    let mut a = view_index::Model::new();
    a.set_title("Bermuda".to_string());
    content::Html(a.render())
    // content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
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
        .mount("/", routes![index, today, about, documents, folders])
        .launch();
}
