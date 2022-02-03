use std::fs::File;
use std::io::prelude::*;

pub fn run() -> Result<String, ()> {
    let mut file = File::open("privkey.txt").expect("cant open file");
    let mut key = String::new();
    file.read_to_string(&mut key)
        .expect("Oops!, cannot read file...");
    Ok(key)
}
