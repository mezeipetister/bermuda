// Demo bin

extern crate core_lib;
use core_lib::{catalog, doc};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut catalog = catalog::init();
    for _i in 1..10 {
        let mut doc = doc::Doc::new();
        doc.set_title("Lorem ipsum dolorem set ami, wohooo na mi a szuti?".to_string());
        doc.set_description("Wohooooo azt a mindenit, ez aztán tényleg kezd működni! Na lehozzuk a csillagokat az égről? Hajrá gyerekek!".to_string());
        doc.save();
        let id = doc.get_id();
        catalog.add_document_to_catalog(doc);
        println!(
            "Document id: {}",
            catalog.get_document_by_id(id.to_string()).get_id()
        );
    }
    println!("Loading took: {}ms", now.elapsed().as_millis());
    println!("Doc count: {}", catalog.get_documents_number());
}
