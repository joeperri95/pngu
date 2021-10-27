use std::str;

pub struct Chunk
{
    pub size: u32,
    pub chunk_type: String,
    pub data: Vec<u8>,
    pub crc32: u32,
}

impl Chunk
{

    pub fn new(chunk_type: [u8;4], data: Vec<u8>) -> Chunk
    {
        let size = (data.len() + 4) as u32; 

        let mut crc_buffer = &mut chunk_type.to_vec();
        crc_buffer.extend(&data); 
        
        println!("{:?}", chunk_type);
        
        Chunk
        {
            size: size,
            chunk_type: str::from_utf8(&chunk_type).unwrap().to_string(),
            data: data,
            crc32: crc32(&crc_buffer),
        }
    }

    pub fn from_buffer(data: &[u8]) -> Chunk
    {
        let mut chunk: Chunk;
        let mut offset = 0;
        let mut chunk_size_data: [u8; 4] = Default::default();
        chunk_size_data.copy_from_slice(&data[offset..(offset + 4 )]);
        let chunk_size = u32::from_be_bytes(chunk_size_data);
        offset += 4;
        
        let chunk_type_data = &data[offset..(offset + 4)];
        let chunk_type = std::str::from_utf8(chunk_type_data).unwrap();
        offset += 4;

        let chunk_data = &data[offset..(offset + chunk_size as usize)];
        let mut data_vec = Vec::new();
        for i in chunk_data.iter()
        {
            data_vec.push(*i);
        }
        offset += chunk_size as usize;

        let mut chunk_crc_data: [u8;4] = Default::default();
        chunk_crc_data.copy_from_slice(&data[offset..(offset + 4)]);
        let chunk_crc = u32::from_be_bytes(chunk_crc_data);
        
        Chunk
        {
            size: chunk_size,
            chunk_type: chunk_type.to_string(),
            data: data_vec, 
            crc32: chunk_crc,
        }
    }

    pub fn to_vec(&self) -> Vec<u8>
    {
        let mut result = Vec::new();
        result.extend((&self.size).to_be_bytes());
        result.extend((&self.chunk_type).as_bytes());
        result.extend(&self.data);
        result.extend((&self.crc32).to_be_bytes());

        result
    }

}

// compute and return the CRC32 of data
fn crc32(data: &[u8]) -> u32
{

    let mut c: u64;
    let mut crc_register: u64 = 0xffffffff;
    let mut crc_table: [u64; 256] = [0;256];

    for i in 0..256 {
        c = i;
        for _j in  0..8
        {
            c = if (c & 1) == 1 {0xedb88320 ^ (c >> 1)} else {c >> 1};
        }

        crc_table[i as usize] = c;
    }

    for i in 0..data.len(){
        crc_register = crc_table[((crc_register ^ (*data.get(i).unwrap() as u64)) & 0xff) as usize] ^ (crc_register >>
        8);
    }

   return (crc_register ^ 0xffffffff) as u32;
}

// Create a png chunk with the secret message
pub fn create_pngu_chunk(message: &String) -> Chunk 
{

    // A tEXt chunk uses a key value scheme separated by a null byte
    // Our secret message is stored under the PNGu keyword
    const KEYWORD_SIZE: usize = 5;
    let mut encoded_message = "PNGu\0".as_bytes().to_vec();

    let chunk_size = (message.len() + KEYWORD_SIZE) as u32;
    let chunk_type: String= "tEXt".to_string();

    encoded_message.extend(message.as_bytes().to_vec());
    let mut crc_buffer: Vec<u8> = Vec::new();

    crc_buffer.extend(&encoded_message);
    let chunk_crc = crc32(&crc_buffer);
    
    Chunk
    {
        size: chunk_size,
        chunk_type: chunk_type,
        data: encoded_message,
        crc32: chunk_crc,
    }
}
