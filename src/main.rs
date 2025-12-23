use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let rom: Vec<u8>;
    if args.len() != 2 {
        eprintln!("Please provide a file.");
        std::process::exit(1);
    }

    match utils::process_file(&args[1]) {
        Ok(file) => rom = file,
        Err(why) => {
            eprintln!("Error opening file '{}': {}", &args[1], why);
            std::process::exit(1);
        }
    };
    drop(args);

    for char in &rom {
        if '\n' == *char as char {
            println!("{} \t .", *char);
        } else {
            println!("{} \t {}", *char, *char as char);
        }
    }

    let sdl = sdl2::init()?;
    let mut display = utils::Display::new(&sdl)?;
    let mut event_pump = sdl.event_pump()?;
    let mut collision: bool;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

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

        collision = display.set_pixel(x as usize, (y + 10) as usize, true);
        y = (y + 1) % 10;
        x = (x + 1) % 64;
        display.draw();
        println!("{}", collision);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    std::process::exit(0);
}

// TODO: Implement some opcode logic?
