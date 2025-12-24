use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::Read;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCALE: u32 = 10;

pub struct Display {
    // Canvas object? Whatever this is
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    // Memory to render
    framebuffer: [bool; 64 * 32],
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
        let video = sdl_context.video()?;
        let window = video
            .window("CHIP-8", SCREEN_WIDTH * SCALE, SCREEN_HEIGHT * SCALE)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Display {
            canvas,
            framebuffer: [false; 64 * 32],
        })
    }

    pub fn clear(&mut self) {
        self.framebuffer = [false; 64 * 32];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) -> bool {
        let index = y * 64 + x;
        let collision = self.framebuffer[index] && on;
        self.framebuffer[index] ^= on;
        collision
    }

    /// Here is a description
    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = (y * SCREEN_WIDTH + x) as usize;
                if self.framebuffer[index] {
                    let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                    // ERROR: I don't think the SCALE arguments are correct
                    self.canvas.fill_rect(rect).unwrap();
                }
            }
        }
        self.canvas.present();
    }
}

pub fn process_file(file_name: &String) -> std::io::Result<Vec<u8>> {
    let file = File::open(file_name)?;
    let mut buffer: Vec<u8> = Vec::new();
    for byte in file.bytes() {
        buffer.push(byte?);
    }
    return Ok(buffer);
}

pub struct CPU {
    pub rom: Vec<u8>,
    v: [u8; 0x10],         // registers
    pc: u16,               // Program counter
    i: u16,                // Address register
    stk: [u8; 0x10],       // Stack
    sp: u8,                // Stack pointer
    t_delay: u8,           // Delay Timer
    t_sound: u8,           // Sound Timer
    key: [u8; 0x10],       // Keypad
    out: [[bool; 64]; 32], // Display
    mem: [u8; 0x1000],     // Memory map:
                           // 0x000 - 0x1FF - Chip 8 interpreter: contains font set
                           // 0x050 - 0x0A0 - Built in 4x5 pixel fot sent (0-F)
                           // 0x200 - 0xFFF - Program and Work RAM
}

// https://en.wikipedia.org/wiki/CHIP-8#Registers
impl CPU {
    fn display_clear() {
        // 00E0
        // TODO: Define function
    }
    fn sub_return() {
        // 00EE
        // TODO: Define function
    }
    /// 1NNN
    fn goto_address(&mut self, left: u8, right: u8) {
        let address = ((left as u16) << 7) | right as u16;
        self.pc = address & 0x0FFF;
    }
    fn sub_call() {
        // 2NNN
    }
    /// 3XNN
    fn skip_eq_mem(&mut self, left: u8, right: u8) {
        let index = (0x0F & left) as usize;
        self.validate_index(index);
        if right == self.v[index] {
            self.pc += 1;
        }
    }
    /// 4XNN
    fn skip_no_eq(&mut self, left: u8, right: u8) {
        let index = (0x0F & left) as usize;
        self.validate_index(index);
        if right != self.v[index] {
            self.pc += 1;
        }
    }
    /// 5XY0
    fn skip_eq_reg(&mut self, left: u8, right: u8) {
        let l = 0x0F & left;
        let r = (0xF0 & right) >> 3;
        if l == r {
            self.pc += 1;
        }
    }
    /// 6XNN
    fn set_x_mem(&mut self, left: u8, right: u8) {
        let index = (0x0F & left) as usize;
        self.validate_index(index);
        self.v[index] = right;
    }
    /// 7XNN
    fn add_x_mem(&mut self, left: u8, right: u8) {
        let index = (left as usize) << 7;
        self.v[index] += right;
    }
    /// 8XY0
    fn set_x_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.v[x as usize] = self.v[y as usize];
    }
    /// 8XY1
    fn set_x_or_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.v[x as usize] |= self.v[y as usize];
    }
    /// 8XY2
    fn set_x_and_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.v[x as usize] &= self.v[y as usize];
    }
    /// 8XY3
    fn set_x_xor_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.v[x as usize] ^= self.v[y as usize];
    }
    // TODO: 8XY4 - 8XYE: manipulates v[F]
    /// 8XY4
    fn set_x_add_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.v[x as usize] += self.v[y as usize];
    }
    /// 8XY5
    fn set_x_sub_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.v[x as usize] -= self.v[y as usize];
    }
    /// 8XY6
    fn set_x_r_shift(&mut self, left: u8, _: u8) {
        let (_, x) = self.split_byte(left);
        self.v[0xf] = self.v[x as usize] & 0x01;
        self.v[x as usize] >>= 1;
    }
    fn set_x_diff_x() {
        // 8XY7
        // TODO: Define function
    }
    fn set_x_l_shift() {
        // 8XYE
        // TODO: Define function
    }
    fn skip_x_not_y() {
        // 9XY0
        // TODO: Define function
    }
    fn set_i_mem() {
        // ANNN
        // TODO: Define function
    }
    fn jump_to_mem() {
        // BNNN
        // TODO: Define function
    }
    fn set_x_rand() {
        // CXNN
        // TODO: Define function
    }
    fn draw() {
        // DXYN
        // TODO: Define function
    }
    fn skip_is_key() {
        // EX9E
        // TODO: Define function
    }
    fn skip_is_not_key() {
        // EXA1
        // TODO: Define function
    }
    fn set_x_delay() {
        // FX07
        // TODO: Define function
    }
    fn set_delay_x() {
        // FX15
        // TODO: Define function
    }
    fn set_sound_x() {
        // FX18
        // TODO: Define function
    }
    fn set_i_add_x() {
        // FX1E
        // TODO: Define function
    }
    fn set_i_sprt_adr() {
        // FX29
        // TODO: Define function
    }
    fn parse_x_to_i() {
        // FX33
        // TODO: Define function
    }
    fn reg_dump() {
        // FX55
        // TODO: Define function
    }
    fn reg_fill() {
        // FX65
        // TODO: Define function
    }
    fn validate_index(&self, index: usize) {
        if index > 16 {
            panic!("Index invalid -> larger than 15: {}", index);
        }
    }
    pub fn new(rom: Vec<u8>) -> CPU {
        CPU {
            rom: rom,
            v: [0; 16],
            pc: 0,
            i: 0,
            stk: [8; 0x10],
            sp: 00,
            t_delay: 0,
            t_sound: 0,
            key: [8; 0x10],
            out: [[false; 64]; 32],
            mem: [8; 0x1000],
        }
    }
    pub fn test_function(&self) {
        self.validate_index(88);
    }
    fn split_byte(&self, byte: u8) -> (u8, u8) {
        let left = (byte & 0xf0) >> 3;
        let right = byte & 0x0f;
        return (left, right);
    }
}
