use std::fs::File;
use std::io::Read;

pub fn process_file(file_name: &String) -> std::io::Result<Vec<u8>> {
    let file = File::open(file_name)?;
    let mut buffer: Vec<u8> = Vec::new();
    for byte in file.bytes() {
        buffer.push(byte?);
    }
    return Ok(buffer);
}
