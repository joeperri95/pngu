use std::io::{Read, Write};
use std::fs::File;
use crate::utils;
use std::str;
use super::chunk::{Chunk, create_pngu_chunk};

// Stuff a message into a PNG text chunk
pub fn encode(input_filename: String, output_filename: String, message: &str) {

    let mut file = File::open(input_filename).expect("Can't open file");

    let path = utils::get_available_filename(&output_filename);
    let mut out_file = File::create(path).expect("could not create file");
    out_file.write_all(&[137,80,78,71,13,10,26,10]).expect("Could not write to file");

    let mut data = Vec::new();

    file.read_to_end(&mut data).expect("Can't read file");

    let mut quit: bool = false;
    let mut offset = 8;

    while !quit
    {
        
        let chunk = Chunk::from_buffer(&data[offset..]).expect("Could not create chunk");
        let chunk_total_size = chunk.size as usize + 3 * 4;
       
        let chunk_type = chunk.chunk_type.clone();
        
        for i in data[offset..(offset + chunk_total_size)].iter(){
            out_file.write_all(&[*i]).expect("Could not write to file");
        }

        if chunk_type == "IHDR"
        {
            println!("Creating secret chunk"); 
            let secret_chunk = create_pngu_chunk(message).expect("Message was empty");            
            let chunk_buf = secret_chunk.to_vec();
            out_file.write_all(&chunk_buf).expect("Could not write secret chunk to file");
        }

        offset += chunk_total_size;
        quit = chunk_type == "IEND"; 
    }
}

// Extracting a message from a file
pub fn decode(filename: String) -> Option<String> {

    if ! utils::is_png(&filename)
    {
        println!("Not a png file");
        return None;
    }

    let mut file = File::open(filename).expect("Can't open file");
    let mut data = Vec::new();

    file.read_to_end(&mut data).expect("Can't read file");

    let mut quit: bool = false;
    let mut offset = 8;

    while !quit
    {
        
        let chunk = Chunk::from_buffer(&data[offset..]).expect("Could not create chunk");
        let chunk_total_size = chunk.size as usize + 3 * 4;
       
        let chunk_type = chunk.chunk_type.clone();        

        if chunk_type == "tEXt"
        {                        
            let ret = str::from_utf8(&chunk.data).unwrap().to_string();
            let msg: Vec<&str> = ret.split('\u{0}').collect(); 
            println!("{:?}", msg[1]);
            return Some(msg[1].to_string()); 
        }

        offset += chunk_total_size;
        quit = chunk_type == "IEND";
    }
    None
}
