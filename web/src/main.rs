#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate core_lib;
use core_lib::{catalog, doc};
use rocket::State;
use std::sync::Mutex;

#[get("/")]
fn index(catalog: State<C>) -> String {
    let mut _catalog = catalog.inner().catalog.lock().unwrap();
    format!("Document number is: {}", _catalog.get_documents_number())
}

#[get("/add")]
fn add(catalog: State<C>) -> String {
    let mut new_doc = doc::Doc::new();
    new_doc.save();
    let mut _catalog = catalog.inner().catalog.lock().unwrap();
    _catalog.add_document_to_catalog(new_doc);
    format!("Ok!")
}

struct C {
    catalog: Mutex<catalog::Catalog>,
}

fn main() {
    let c = C {
        catalog: Mutex::new(catalog::init()),
    };
    rocket::ignite()
        .manage(c)
        .mount("/", routes![index, add])
        .launch();
}
