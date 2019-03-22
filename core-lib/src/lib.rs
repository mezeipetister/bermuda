#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

// Rust documentation
pub mod files;
pub mod doc;
pub mod catalog;

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns a timestamp as a string
pub fn get_timestamp_as_micros() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}",since_the_epoch.as_micros())
}
