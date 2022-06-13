use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use futures::executor::block_on;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    let path_dir = path.parent();
    let path_name = path.file_stem();
    let path_ext = path.extension();

    println!("parent: {:?}", path_dir);
    println!("name: {:?}", path_name);
    println!("ext: {:?}", path_ext);
    println!("path: {:?}", path.to_str());

    let filename = String::from(path.to_str().unwrap());
    println!("Processing target: {}", filename);
    
    let metadata = fs::metadata(&filename).expect("metadata not found");
    let file_length: usize = metadata.len() as usize;
    println!("File Length: {}", file_length);

    let future_read_image = read_file_as_byte(&filename, file_length);
    match block_on(future_read_image) {
        Ok(buffer) => {
            println!("Success: {}", buffer.len());
            let copy_name = format!("{}/{}_{}.{}", path_dir, path_name, "new", path_ext);
            let future_copy_image = save_file(&buffer, copy_name);
            match block_on(future_copy_image) {
                Ok(result) => {

                },
                Err(e_copy) => {
                    println!("save failed: {}", e);
                }
            }

        },
        Err(e) => {
            println!("read failed: {}", e);
        
    }
}

async fn read_file_as_byte(filename: &String, filesize: usize) -> io::Result<Vec<usize>>{
    
    let mut f = File::open(filename).expect("file not found");
    let mut buffer = Vec::new<usize>();

    f.read_to_end(&mut buffer).expect("error while reading file");
    
    Ok(buffer)
}

async fn save_file(in_buffer: &Vec<usize>, in_filename: &String) -> io::Result<bool>) {

}