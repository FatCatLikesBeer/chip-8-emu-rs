use rand::Rng;
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
        let address = ((left as u16) << 8) | right as u16;
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
        let r = (0xF0 & right) >> 4;
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
        let index = (left as usize) << 8;
        self.validate_index(index);
        self.v[index] += right;
    }
    /// 8XY0
    fn set_x_y(&mut self, left: u8, right: u8) {
        let (_, x): (usize, usize) = self.split_byte(left);
        let (y, _): (usize, usize) = self.split_byte(right);
        self.validate_index(x);
        self.validate_index(x);
        self.v[x] = self.v[y];
    }
    /// 8XY1
    fn set_x_or_y(&mut self, left: u8, right: u8) {
        let (_, x): (usize, usize) = self.split_byte(left);
        let (y, _): (usize, usize) = self.split_byte(right);
        self.validate_index(x);
        self.validate_index(y);
        self.v[x] |= self.v[y];
    }
    /// 8XY2
    fn set_x_and_y(&mut self, left: u8, right: u8) {
        let (_, x): (usize, usize) = self.split_byte(left);
        let (y, _): (usize, usize) = self.split_byte(right);
        self.validate_index(x);
        self.validate_index(y);
        self.v[x] &= self.v[y];
    }
    /// 8XY3
    fn set_x_xor_y(&mut self, left: u8, right: u8) {
        let (_, x): (usize, usize) = self.split_byte(left);
        let (y, _): (usize, usize) = self.split_byte(right);
        self.validate_index(x);
        self.validate_index(y);
        self.v[x] ^= self.v[y];
    }
    // TODO: 8XY4 - 8XYE: manipulates v[F]
    /// 8XY4
    fn set_x_add_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.validate_index(x);
        self.validate_index(y);
        self.v[x as usize] += self.v[y as usize];
    }
    /// 8XY5
    fn set_x_sub_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.validate_index(x);
        self.validate_index(y);
        self.v[x as usize] -= self.v[y as usize];
    }
    /// 8XY6
    fn set_x_r_shift(&mut self, left: u8, _: u8) {
        let (_, x): (usize, usize) = self.split_byte(left);
        self.validate_index(x);
        self.v[0xf] = self.v[x] & 0x01;
        self.v[x] >>= 1;
    }
    /// 8XY7
    fn set_x_diff_x(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.validate_index(x);
        self.validate_index(y);
        self.v[x] -= self.v[y];
    }
    /// 8XYE
    fn set_x_l_shift(&mut self, left: u8, _: u8) {
        let (_, x) = self.split_byte(left);
        self.validate_index(x);
        self.v[x as usize] <<= 1;
    }
    /// 9XY0
    fn skip_x_not_y(&mut self, left: u8, right: u8) {
        let (_, x) = self.split_byte(left);
        let (y, _) = self.split_byte(right);
        self.validate_index(x);
        self.validate_index(y);
        if self.v[x] == self.v[y] {
            self.pc += 1;
        }
    }
    /// ANNN
    fn set_i_mem(&mut self, left: u8, right: u8) {
        let (_, n1): (u8, u8) = self.split_byte(left);
        let (n2, n3): (u8, u8) = self.split_byte(right);
        self.i = self.smash_3_nib(n1, n2, n3)
    }
    /// BNNN
    fn jump_to_mem(&mut self, left: u8, right: u8) {
        let (_, n1): (u8, u8) = self.split_byte(left);
        let (n2, n3): (u8, u8) = self.split_byte(right);
        self.pc = (self.v[0] as u16) + self.smash_3_nib(n1, n2, n3);
    }
    /// CXNN
    /// Vx = rand() & NN
    fn set_x_rand(&mut self, left: u8, right: u8) {
        let mut rng = rand::rng();
        let quad: (u8, u8, u8, u8) = self.split_pair((left, right));
        let nn = self.smash_2_nib(quad.2, quad.3);
        let rand: u8 = rng.random();
        self.v[quad.1 as usize] = rand & nn;
    }
    /// DXYN
    fn draw(&mut self, left: u8, right: u8) {
        let q: (u8, u8, u8, u8) = self.split_pair((left, right));
        // self.draw(q1, q2, q3);
        // TODO: Define draw method
        // WARN: This function has not been defined
    }
    /// EX9E
    /// if (key() = vx)
    fn skip_is_key(&mut self, left: u8, right: u8) {
        let q: (u8, u8, u8, u8) = self.split_pair((left, right));
        // Skips the next instruction if the key stored in VX(only consider the lowest nibble)
        // is pressed (usually the next instruction is a jump to skip a code block)
        // TODO: What the hell is pressed?
        // TODO: Define draw method
        // WARN: This function has not been defined
    }
    fn skip_is_not_key(&mut self, left: u8, right: u8) {
        // EXA1
        // TODO: Define function
    }
    fn set_x_delay(&mut self, left: u8, right: u8) {
        // FX07
        // TODO: Define function
    }
    fn set_delay_x(&mut self, left: u8, right: u8) {
        // FX15
        // TODO: Define function
    }
    fn set_sound_x(&mut self, left: u8, right: u8) {
        // FX18
        // TODO: Define function
    }
    fn set_i_add_x(&mut self, left: u8, right: u8) {
        // FX1E
        // TODO: Define function
    }
    fn set_i_sprt_adr(&mut self, left: u8, right: u8) {
        // FX29
        // TODO: Define function
    }
    fn parse_x_to_i(&mut self, left: u8, right: u8) {
        // FX33
        // TODO: Define function
    }
    fn reg_dump(&mut self, left: u8, right: u8) {
        // FX55
        // TODO: Define function
    }
    fn reg_fill(&mut self, left: u8, right: u8) {
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
    fn split_byte<T>(&self, byte: u8) -> (T, T)
    where
        T: From<u8>,
    {
        let left = (byte & 0xf0) >> 4;
        let right = byte & 0x0f;
        return (T::from(left), T::from(right));
    }
    /// Takes nibbles L and nibble R
    /// Returns a u8 0xLR
    fn smash_2_nib(&self, left: u8, right: u8) -> u8 {
        (left << 4) | right
    }
    /// Takes nibble L, nibble M, nibble R
    /// Returns u16 0x0LMR;
    fn smash_3_nib(&self, left: u8, middle: u8, right: u8) -> u16 {
        (left as u16) << 8 | (middle as u16) << 4 | (right as u16)
    }
    fn split_pair<T>(&self, pair: (u8, u8)) -> (T, T, T, T)
    where
        T: From<u8>,
    {
        let (n1, n2): (T, T) = self.split_byte(pair.0);
        let (n3, n4): (T, T) = self.split_byte(pair.1);
        (n1, n2, n3, n4)
    }
}
