use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Hello, world! This is Joseph's gcode checker.");

    // create a path to the file
    let path = Path::new("/Users/jo/rust/rust_snippets/test_files/9486f49313473344b9c78fb41f2dec58.gcode");
    let display = path.display();

    // open the file in read-only mode
    let file = match File::open(&path) {
        // description method of error returns string describing the error
        Err(e) => panic!("could not open {}: {}", display, Error::description(&e)),
        Ok(file) => file
    };

    // generate an iterator out of the file
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for l in lines {
        println!("{}", l.unwrap_or_else(|e| panic!("My message {}", e)));
    }
}
