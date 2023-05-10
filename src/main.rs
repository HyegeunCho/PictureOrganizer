use std::env;
use std::fs;
// use std::io;
// use std::fs::File;
// use std::io::Read;
//use futures::executor::block_on;
use image;
use image::GenericImageView;
use std::path::Path;

use chrono::offset::Utc;
use chrono::DateTime;

// async fn read_file_as_byte(filename: &String, filesize: usize) -> io::Result<Vec<u8>>{
    
//     let mut f = File::open(filename).expect("file not found");
//     let mut buffer = vec![0; filesize];

//     f.read_to_end(&mut buffer).expect("error while reading file");
    
//     Ok(buffer)
// }

// fn read_exif(inPath: &String) -> Result<bool, String> {
//     let file = std::fs::File::open(inPath).expect("file not found");
//     let mut bufreader = std::io::BufReader::new(&file);
//     let exifreader = exif::Reader::new();
//     let exif = exifreader.read_from_container(&mut bufreader).unwrap();
//     for f in exif.fields() {
//         println!("{} {} {}",
//                  f.tag, f.ifd_num, f.display_value().with_unit(&exif));
//     }

//     Ok(true)
// }



fn main() {

    let args: Vec<String> = env::args().collect();

    let target_path = &args[1];
    println!("Processing target: {}", target_path);

    let datetime = get_created_datetime(&target_path);
    println!("created: {}", datetime.format("%Y/%m/%d %T"));

    let path = Path::new(target_path);
    let mut file_stem_val: String = path.file_stem().unwrap().to_string_lossy().as_ref().to_string();
    let file_extension_val = path.extension().unwrap().to_string_lossy();

    let new_parent_path = format!("classified/{}", datetime.format("%Y_%m_%d"));
    let is_newdir_exist = Path::new(&new_parent_path).is_dir();
    if !is_exist_dir(&new_parent_path) {
        fs::create_dir_all(&new_parent_path).expect("Cannot create directory");
    }

    let mut new_path = format!("{}/{}.{}", new_parent_path, file_stem_val, file_extension_val);
    println!("New path: {}", new_path);

    while is_exist_file(&new_path) {
        file_stem_val.push_str("_1");
        println!("changed file_stem: {}", file_stem_val);
        new_path = format!("{}/{}.{}", new_parent_path, file_stem_val, file_extension_val);
    }

    if is_exist_file(&new_path) {
        println!("EXIST {}", new_path);
    }
    else {
        println!("NOT EXIST {}", new_path);
    }

    let img = image::open(target_path).unwrap();
    let (w, h) = img.dimensions();
    println!("dimesion: ({}, {})", w, h);   
    
    img.save(new_path).expect("Cannot save copy image.");

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

fn get_created_datetime(in_path: &String) -> DateTime<Utc> {

    let metadata = fs::metadata(in_path).expect("metadata not found");
    let systemtime = metadata.created().unwrap();
    let datetime: DateTime<Utc> = systemtime.into();

    datetime
}

fn is_exist_file(in_path: &String) -> bool {

    Path::new(&in_path).is_file()
}

fn is_exist_dir(in_path: &String) -> bool {
    
    Path::new(&in_path).is_dir()
}