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

pub struct CPU {
    pub v: [u8; 0x10],         // registers
    pub pc: u8,                // Program coutner
    pub i: u16,                // Address register
    pub stk: [u8; 0x10],       // Stack
    pub sp: u8,                // Stack pointer
    pub t_delay: u8,           // Delay Timer
    pub t_sound: u8,           // Sound Timer
    pub key: [u8; 0x10],       // Keypad
    pub out: [[bool; 64]; 32], // Display
    pub mem: [u8; 0x1000],     // Memory map:
                               // 0x000 - 0x1FF - Chip 8 interpreter: contains font set
                               // 0x050 - 0x0A0 - Built in 4x5 pixel fot sent (0-F)
                               // 0x200 - 0xFFF - Program and Work RAM
}

// https://en.wikipedia.org/wiki/CHIP-8#Registers

impl CPU {
    pub fn display_clear() {
        // 00E0
    }
    pub fn sub_return() {
        // 00EE
    }
    pub fn goto_add() {
        // 1NNN
    }
    pub fn sub_call() {
        // 2NNN
    }
    pub fn skip_eq_mem() {
        // 3XNN
    }
    pub fn skip_no_eq() {
        // 4XNN
    }
    pub fn skip_eq_reg() {
        // 5XY0
    }
    pub fn set_x_mem() {
        // 6XNN
    }
    pub fn add_x_mem() {
        // 7XNN
    }
    pub fn set_x_y() {
        // 8XY0
    }
    pub fn set_x_or_y() {
        // 8XY1
    }
    pub fn set_x_and_y() {
        // 8XY2
    }
    pub fn set_x_xor_y() {
        // 8XY3
    }
    pub fn set_x_add_y() {
        // 8XY4
    }
    pub fn set_x_sub_y() {
        // 8XY5
    }
    pub fn set_x_r_shift() {
        // 8XY6
    }
    pub fn set_x_diff_x() {
        // 8XY7
    }
    pub fn set_x_l_shift() {
        // 8XYE
    }
    pub fn skip_x_not_y() {
        // 9XY0
    }
    pub fn set_i_mem() {
        // ANNN
    }
    pub fn jump_to_mem() {
        // BNNN
    }
    pub fn set_x_rand() {
        // CXNN
    }
    pub fn draw() {
        // DXYN
    }
    pub fn skip_is_key() {
        // EX9E
    }
    pub fn skip_is_not_key() {
        // EXA1
    }
    pub fn set_x_delay() {
        // FX07
    }
    pub fn set_delay_x() {
        // FX15
    }
    pub fn set_sound_x() {
        // FX18
    }
    pub fn set_i_add_x() {
        // FX1E
    }
    pub fn set_i_sprt_adr() {
        // FX29
    }
    pub fn parse_x_to_i() {
        // FX33
    }
    pub fn reg_dump() {
        // FX55
    }
    pub fn reg_fill() {
        // FX65
    }
}
