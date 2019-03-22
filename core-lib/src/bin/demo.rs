// Demo bin

extern crate core_lib;
use core_lib::catalog;
use core_lib::doc;
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    let now = Instant::now();
    let mut catalog = catalog::init();
    // for i in 1..10000 {
    //     let mut doc = doc::Doc::new();
    //     doc.set_title("Lorem ipsum dolorem set ami, wohooo na mi a szuti?".to_string());
    //     doc.set_description("Wohooooo azt a mindenit, ez aztán tényleg kezd működni! Na lehozzuk a csillagokat az égről? Hajrá gyerekek!".to_string());
    //     doc.save();
    //     catalog.add_document_to_catalog(doc);
    // }
    // for item in catalog.get_documents() {
    //     println!("Doc ID: {}",item.get_id());
    // }
    println!("Loading took: {}ms", now.elapsed().as_millis());
    println!("Doc count: {}", catalog.get_documents_number());
}
