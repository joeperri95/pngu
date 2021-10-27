use std::fs::{File, rename, remove_file};
use std::io::Read;
use std::path::Path;
use std::env;
use image;

// Check if filename is available. If it is not try again with an appended number
pub fn get_available_filename(filename: &str) -> String
{
    let mut path = env::current_dir().expect("can't access current directory");
    let mut number = 1;
    let filestem = Path::new(filename).file_stem().unwrap().to_str().unwrap();

    path.push("output");
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
    downloaded_filename.to_string()
}

// using the image crate convert a downloaded jpeg file to a png
pub fn convert_jpg_to_png(filename: &str)
{
    if is_png(filename)
    {
        println!("Nothing to do");
        return;
    }
    else if is_jpeg(filename)
    {
        // can have a jpeg image with a .png extension. Handle this case
        let basepath = Path::new(filename).parent().unwrap().to_str().unwrap().to_string();
        let filestem = Path::new(filename).file_stem().unwrap().to_str().unwrap().to_string();
        let jpeg_path = basepath.clone() + "/" + &filestem +".jpg";
        let png_path = basepath.clone() + "/" + &filestem + ".png";
        
        rename(&filename, &jpeg_path);
        let img = image::open(&jpeg_path).unwrap();
        img.save(png_path); 
        remove_file(&jpeg_path);
    }
    else
    {
        println!("Not a jpeg or png file");
    }
}

// Check if the file has the jpeg signature
pub fn is_jpeg(filename: &str) -> bool
{
    // jpeg starts with a 2 byte marker that is 0xFF followed by a byte that is not 0xFF or 0x00

    let mut file = File::open(filename).expect("Can't open file");
    let mut data: [u8; 2] = [0;2];
    file.read(&mut data).expect("Can't read file");
    
    if (data[0] == 0xFF) && (data[1] != 0xFF) && (data[1] != 0x00)
    {
        true
    }
    else
    {
        false
    }
}

// Check if the file has the png signature
pub fn is_png(filename: &str) -> bool
{

    let mut file = File::open(filename).expect("Can't open file");
    let mut data: [u8; 8] = [0;8];
    const SIGNATURE: [u8;8] = [137,80,78,71,13,10,26,10];
    let mut ret = true;

    file.read(&mut data).expect("Can't read file");

    for i in 0..8
    {
        ret = if data[i] == SIGNATURE[i] {true} else {false};
    }

    ret
}
