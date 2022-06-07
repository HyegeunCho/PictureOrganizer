use std::env;
use std::fs;
use std::io;
use std::fs::File;
use std::io::Read;
use futures::executor::block_on;

async fn read_file_as_byte(filename: &String, filesize: usize) -> io::Result<Vec<u8>>{
    
    let mut f = File::open(filename).expect("file not found");
    let mut buffer = vec![0; filesize];

    f.read_to_end(&mut buffer).expect("error while reading file");
    
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Processing target: {}", filename);

    
    let metadata = fs::metadata(filename).expect("metadata not found");
    let file_length: usize = metadata.len() as usize;
    println!("File Length: {}", file_length);

    let future = read_file_as_byte(&filename, file_length);
    match block_on(future) {
        Ok(buffer) => {
            println!("Success: {}", buffer.len());            
        },
        Err(e) => {
            println!("read failed: {}", e);
        }
    }
}

