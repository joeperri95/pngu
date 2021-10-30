use std::io::{Read, Write};
use std::fs::File;

use crate::utils;

use super::chunk::{Chunk, create_pngu_chunk};

pub fn process_png(input_filename: String, output_filename: String, message: &str) {

    if ! utils::is_png(&input_filename) {
        return;
    };

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
        
        let chunk = Chunk::from_buffer(&data[offset..]);
        let chunk_total_size = chunk.size as usize + 3 * 4;
       
        let chunk_type = chunk.chunk_type.clone();
        
        for i in data[offset..(offset + chunk_total_size)].iter(){
            out_file.write_all(&[*i]).expect("Could not write to file");
        }

        if chunk_type == "IHDR"
        {
            println!("Creating secret chunk"); 
            let secret_chunk = create_pngu_chunk(message);            
            let chunk_buf = secret_chunk.to_vec();
            out_file.write_all(&chunk_buf).expect("Could not write secret chunk to file");
        }

        offset += chunk_total_size;
        quit = chunk_type == "IEND"; 
    }
}
