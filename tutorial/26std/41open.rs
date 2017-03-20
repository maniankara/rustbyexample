use std::path::Path;
use std::fs::File;

use std::io::prelude::*;

static LICENSE: & 'static str= "/home/anoop/Work/github/rustbyexample/LICENSE";

fn main() {

    //path
    let path = Path::new(LICENSE);

    // open
    let mut file = match File::open(&path) { // File::create(&path) // for writing

        Err(e) => panic!("Failed to open: {:?}", path),

        Ok(file) => file
    };

    // read
    let mut cont = String::new();
    // match file.write_all(str.as_bytes) // for writing
    match file.read_to_string(&mut cont) {

        Err(e) => panic!("Unable to read: {:?}", path),

        Ok(_) => println!("")
    };

    //display
    println!("File contents: {:?}", cont);
}