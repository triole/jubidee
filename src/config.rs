use serde_yaml;
use std::collections::HashMap;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

pub fn read(filename: &str) -> HashMap<String, u32> {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&filename) {
        Err(err) => {
            println!("Couldn't open {}: {}", display, err.description());
            process::exit(0x0101);
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("something went wrong reading the file");

    let config: HashMap<String, u32> = serde_yaml::from_str(&s).unwrap();
    return config;
}
