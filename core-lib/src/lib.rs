#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

// Rust documentation
pub mod catalog;
pub mod doc;
pub mod files;

use std::time::{SystemTime, UNIX_EPOCH};

//  ____                                _
// |  _ \                              | |
// | |_) | ___ _ __ _ __ ___  _   _  __| | __ _
// |  _ < / _ \ '__| '_ ` _ \| | | |/ _` |/ _` |
// | |_) |  __/ |  | | | | | | |_| | (_| | (_| |
// |____/ \___|_|  |_| |_| |_|\__,_|\__,_|\__,_|

/// Returns a timestamp as a string
pub fn get_timestamp_as_micros() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}", since_the_epoch.as_micros())
}
