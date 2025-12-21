use std::fs::File;
use std::io::Read;

fn process_file(file_name: &String) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    drop(file);
    return Ok(buffer);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Please provide a file.");
        std::process::exit(1);
    }

    let file_contents = match process_file(&args[1]) {
        Ok(file) => file,
        Err(why) => {
            eprintln!("Error opening file '{}': {}", &args[1], why);
            std::process::exit(1);
        }
    };

    print!("{}", file_contents);

    std::process::exit(0);
}

// TODO: Binary accepts arguments.
// Argument is a file name
// Open and verify file name
// Store file name
// TODO: Parse file by lines
// TODO: Run some logic based on lines
