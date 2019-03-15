extern crate chrono;

use std::io;
use chrono::{Utc};

fn get_time() {
    println!("Time is {}", Utc::now());
}

fn main() {
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Err(error) => println!("Error, oo: {}", error),
            _ => (),
        }

        match buffer.trim() {
            "exit" => break,
            "hi" => println!("Hello"),
            "time" => get_time(),
            _ => println!("Unkown command"),
        }
    }

    println!("Oo exited by user!");
}
