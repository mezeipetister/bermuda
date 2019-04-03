#![feature(proc_macro_hygiene, decl_macro, plugin)]

pub mod widgets;
pub mod templater;
pub mod views;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

extern crate core_lib;
use core_lib::catalog;
use rocket::response::content;
use rocket::State;
use std::sync::Mutex;

use widgets::Widgets;
use views::Html;

#[get("/")]
fn index(_c: State<View>) -> content::Html<String> {
    content::Html(_c.inner().views.lock().unwrap().demo_document_plain_view())
}

#[get("/add")]
fn add(_c: State<View>) -> content::Html<String> {
    content::Html(_c.inner().views.lock().unwrap().demo_add_new_document())
}

struct View {
    views: Mutex<Html>,
}

fn main() {
    let catalog = catalog::init();
    let widgets = Widgets::new(catalog);
    let views = Html::new();

    let c = View {
        views: Mutex::new(views),
    };

    rocket::ignite()
        .manage(c)
        .mount("/", routes![index, add])
        .launch();
}
