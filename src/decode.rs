// Implementation for extracting the message from a file

use std::str;
use std::fs::{File};
use std::io::{Read };
use crate::utils;
use crate::chunk;

pub fn process_png(filename: String) -> String {

    if ! utils::is_png(&filename) {
        return "".to_string();
    };

    let mut file = File::open(filename).expect("Can't open file");
    let mut data = Vec::new();

    file.read_to_end(&mut data).expect("Can't read file");

    let mut quit: bool = false;
    let mut offset = 8;

    while !quit
    {
        
        let chunk = chunk::Chunk::from_buffer(&data[offset..]);
        let chunk_total_size = chunk.size as usize + 3 * 4;
       
        let chunk_type = chunk.chunk_type.clone();        

        if chunk_type == "tEXt"
        {                        
            let ret = str::from_utf8(&chunk.data).unwrap().to_string();
            let mut msg: Vec<&str> = ret.split("\u{0}").collect(); 
            println!("{:?}", msg[1]);
            return msg[1].to_string(); 
        }

        offset += chunk_total_size;
        quit = if chunk_type == "IEND" {true} else {false};
    }

    "".to_string()
}
