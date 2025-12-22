mod utils;

fn main() {
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

    std::process::exit(0);
}

// TODO: Binary accepts arguments.
// Argument is a file name
// Open and verify file name
// Store file name
// TODO: Parse file by lines
// TODO: Run some logic based on lines
