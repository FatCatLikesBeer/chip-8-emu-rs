use sdl2::pixels::Color;
use sdl2::rect::Rect;

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
