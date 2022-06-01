use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Processing target: {}", filename);

    let mut f = File::open(filename).expect("file not found");
    
}
