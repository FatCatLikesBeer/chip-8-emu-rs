mod utils;

fn main() {
    let mut cpu = utils::Explore { acc: 0 };
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

    for command in rom.windows(2) {
        if command[0] == 0x10 {
            cpu.add(command[1] as i32);
            continue;
        }
        if command[0] == 0x20 {
            cpu.shift();
            continue;
        }
        if command[0] == 0x30 {
            cpu.print();
            continue;
        }
    }

    std::process::exit(0);
}

// TODO: Run some logic based on lines
