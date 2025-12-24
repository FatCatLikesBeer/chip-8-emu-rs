use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let rom: Vec<u8>;
    // let args = std::env::args().collect::<Vec<String>>();
    // if args.len() != 2 {
    //     eprintln!("Please provide a file.");
    //     std::process::exit(1);
    // }
    //
    // match utils::process_file(&args[1]) {
    //     Ok(file) => rom = file,
    //     Err(error) => {
    //         eprintln!("Error opening file '{}': {}", &args[1], error);
    //         std::process::exit(1);
    //     }
    // };
    // drop(args);
    // // None of the above is nessary when no file is being read

    // Init CPU & Display
    let cpu: utils::CPU;
    let sdl = sdl2::init()?;
    let mut display = utils::Display::new(&sdl)?;
    let mut event_pump = sdl.event_pump()?;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut index: i8 = 0;

    'running: loop {
        // Exit stuff
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q | Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        display.set_pixel(x as usize, (y + 10) as usize, true);
        index = (index + 1) % 64;
        if index == 0 {
            display.clear();
        }
        y = (y + 1) % 10;
        x = (x + 1) % 64;
        display.draw();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    std::process::exit(0);
}

// TODO: Integrate display into CPU
// TODO: Implement some opcode logic?
