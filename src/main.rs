use std::env;
use std::fs;
use std::io;
use std::fs::File;
use std::io::Read;
//use futures::executor::block_on;
use image;
use image::GenericImageView;
use std::path::Path;


async fn read_file_as_byte(filename: &String, filesize: usize) -> io::Result<Vec<u8>>{
    
    let mut f = File::open(filename).expect("file not found");
    let mut buffer = vec![0; filesize];

    f.read_to_end(&mut buffer).expect("error while reading file");
    
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let path = Path::new(filename);

    println!("Processing target: {}", filename);
    
    let is_newdir_exist = Path::new("classified").is_dir();
    if !is_newdir_exist {
        fs::create_dir_all("classified");
    }

    let new_filename = format!("classified/{}.{}", path.file_stem().unwrap().to_string_lossy(), path.extension().unwrap().to_string_lossy());
    println!("New filename: {}", new_filename);

    let img = image::open(filename).unwrap();
    let (w, h) = img.dimensions();
    println!("dimesion: ({}, {})", w, h);   
    


    let metadata = fs::metadata(filename).expect("metadata not found");
    println!("created: {:?}", metadata.created());
    
    img.save(new_filename);

    // let file_length: usize = metadata.len() as usize;
    // println!("File Length: {}", file_length);
    // println!("created: {:?}", metadata.created());
   

    // let future = read_file_as_byte(&filename, file_length);
    // match block_on(future) {
    //     Ok(buffer) => {
    //         println!("Success: {}", buffer.len());            
    //     },
    //     Err(e) => {
    //         println!("read failed: {}", e);
    //     }
    // }
}

