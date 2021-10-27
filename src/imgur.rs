use rand::prelude::*;
use std::fs::{File};
use std::{thread, time};

use crate::utils;

// Construct a url for a random imgur image
pub fn get_random_imgur_url() -> String
{

     let chars = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
     let mut url = "https://imgur.com/".to_string();
     let mut rng = thread_rng();
 
     for _i in 0..5
     {
         url += &chars.get(rng.gen_range(0..chars.len())).unwrap().to_string();
     }

     // More jpg hits than png
     // Most png files get transcoded to jpeg anyway

     url += ".jpg";
     url
}

// download a random imgur file  
pub fn download_random_imgur_file(filename: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let mut download = File::create(filename)?;
    let mut req = reqwest::blocking::get(get_random_imgur_url())?;

    // bad links will redirect to removed.png
    while req.url().path() == "/removed.png"
    {
        req = reqwest::blocking::get(get_random_imgur_url())?;

        // sleep for a bit to be nice to imgur
        thread::sleep(time::Duration::from_millis(100));
    }

    req.copy_to(&mut download)?;
    Ok(())
}

// get a random image from imgur and convert it to png if necessary
pub fn get_random_png() -> String
{

    let downloaded_filename = utils::get_available_filename("image.png");
    
    loop{
        download_random_imgur_file(&downloaded_filename).expect("Could not get file from imgur");

        if ! utils::is_png(&downloaded_filename.to_string())
        {
            if utils::is_jpeg(&downloaded_filename)
            {
               println!("Converting {} jpeg to png", &downloaded_filename);
               utils::convert_jpg_to_png(&downloaded_filename);
               break;
            }
            else
            {
                println!("Invalid PNG file");
            }
        }
        else
        {
            println!("Good png file");
            break;
        }
    }

    downloaded_filename.to_string()
}
