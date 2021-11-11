use std::fs::{File, create_dir};
use std::io::Read;
use std::path::Path;
use std::env;
use image::io::Reader;

// Check if filename is available. If it is not try again with an appended number
pub fn get_available_filename(filename: &str) -> String
{
    let mut path = env::current_dir().expect("can't access current directory");
    let mut number = 1;
    let filestem = Path::new(filename).file_stem().unwrap().to_str().unwrap();

    path.push("output");
    if ! path.is_dir()
    {
        create_dir(&path).expect("Could not create output dir");
    }

    path.push(filename);
    loop 
    {
        if ! path.is_file()
        {
            break;
        }
        
        path.pop();
        path.push(format!("{}-{}.png", filestem, number));
        number += 1;
    }

    let downloaded_filename = format!("{}", path.display());
    downloaded_filename
}

// using the image crate convert a downloaded jpeg file to a png
pub fn convert_to_png(filename: &str)
{
    // TODO make this return a result and handle the errors at a higher level 
    
    // can have a different format image with a .png extension. Handle this case
    let basepath = Path::new(filename).parent().unwrap().to_str().unwrap().to_string();
    let filestem = Path::new(filename).file_stem().unwrap().to_str().unwrap().to_string();

    // final path 
    let png_path = basepath + "/" + &filestem + ".png";
    
    let img = Reader::open(&filename).expect("Could not open file").with_guessed_format().expect("Could not determine image format").decode().expect("Could not decode image");

    img.save(png_path).expect("Could not save file"); 
}

// check if file is PNG from signature
pub fn is_png(filename: &str) -> bool {
    let mut file = File::open(&filename).expect("Could not open file");
    let mut buffer: [u8;8] = Default::default();
    
    file.read_exact(&mut buffer).expect("Error reading file to buffer");
    buffer.starts_with(b"\x89PNG\r\n\x1a\n")
}
