// Demo bin

extern crate core_lib;
use core_lib::catalog;
use core_lib::doc;

fn main() {
    let catalog = catalog::init();
    let mut d1 = doc::Doc::new();
    d1.set_title("Hello bello".to_string());
    d1.set_description("Kukukukukukukuuuu".to_string());
    d1.save();
}
