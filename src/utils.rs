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
    pub v: [u8; 0x10],         // registers
    pub pc: u16,               // Program counter
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
    /// 1NNN
    fn goto_address(mut self, left: u8, right: u8) {
        let address = ((left as u16) << 7) | right as u16;
        self.pc = address & 0x0FFF;
    }
    pub fn sub_call() {
        // 2NNN
    }
    /// 3XNN
    fn skip_eq_mem(mut self, left: u8, right: u8) {
        let index = (0x0F & left) as usize;
        if index > 15 {
            // ERROR
        }
        if right == self.v[index] {
            self.pc += 1;
        }
    }
    /// 4XNN
    fn skip_no_eq(mut self, left: u8, right: u8) {
        let index = (0x0F & left) as usize;
        if index > 15 {
            // ERROR
        }
        if right != self.v[index] {
            self.pc += 1;
        }
    }
    /// 5XY0
    fn skip_eq_reg(mut self, left: u8, right: u8) {
        let l = 0x0F & left;
        let r = (0xF0 & right) >> 3;
        if l == r {
            self.pc += 1;
        }
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
