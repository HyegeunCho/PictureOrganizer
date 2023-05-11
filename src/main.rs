use std::env;
use std::fs;
use image::*;
use std::path::Path;

use chrono::offset::Utc;
use chrono::DateTime;

use exif;




fn main() {

    let args: Vec<String> = env::args().collect();

    let target_path = &args[1];
    println!("Processing target: {}", target_path);

    let datetime = get_created_datetime(&target_path);
    println!("created: {}", datetime.format("%Y/%m/%d %T"));

    let path = Path::new(target_path);
    let file_stem_val: String = path.file_stem().unwrap().to_string_lossy().as_ref().to_string();
    let file_extension_val = path.extension().unwrap().to_string_lossy();

    let new_parent_path = format!("classified/{}", datetime.format("%Y_%m_%d"));
    let is_newdir_exist = Path::new(&new_parent_path).is_dir();
    if !is_exist_dir(&new_parent_path) {
        fs::create_dir_all(&new_parent_path).expect("Cannot create directory");
    }

    let mut new_path = format!("{}/{}.{}", new_parent_path, file_stem_val, file_extension_val);
    println!("New path: {}", new_path);

    let mut postfix_id = 0;
    while is_exist_file(&new_path) {
        postfix_id += 1;
        let new_file_stem = format!("{}_{}", file_stem_val, postfix_id);
        println!("changed file_stem: {}", new_file_stem);
        new_path = format!("{}/{}.{}", new_parent_path, new_file_stem, file_extension_val);
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
    

    // read exif data
    let file = std::fs::File::open(path).expect("Cannot open file.");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader);

    match exif {
        Ok(exif_val) => {
            for f in exif_val.fields() {
                println!("{} {} {}",
                         f.tag, f.ifd_num, f.display_value().with_unit(&exif_val));
            }
        },
        Err(error) => {
            println!("Could not read exif data: {}", error);
        }
    }
    

    // TEST
    img.save(new_path).expect("Cannot save copy image.");
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