// #![allow(unused_must_use)] // ignore must use result
// #![allow(unused_imports)]  // ignore unused imports
#![allow(dead_code)]  // ignore unused functions


use std::fs::File;


pub(crate) fn read_file(filename: &str) {
    let f = File::open(filename).expect("file not found");  // Open the file
    println!("{:?}", f);  // Print the file


}

