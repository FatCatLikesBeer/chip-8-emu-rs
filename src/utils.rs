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

pub struct Explore {
    pub acc: i32,
}

impl Explore {
    pub fn add(&mut self, val: i32) {
        self.acc = self.acc + val;
    }
    pub fn shift(&mut self) {
        self.acc = self.acc >> 1;
    }
    pub fn print(&self) {
        println!("{}", self.acc);
    }
}
