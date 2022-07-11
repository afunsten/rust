use std::fs;
use std::process;
use std::collections::HashMap;

// This function borrows a slice of the args vector
pub fn analyze_args(args: &[String])-> HashMap<&str, &str> {
    let mut filename = "./config.json";

    let args_length = args.len();
    let mut i = 0;
    for argument in args {
        if argument == "--configfile"  {
            let nexti = i+1;
            if nexti <= args_length-1 {
                //println!("{}", args[nexti]);
                filename = &args[nexti];
            } else {
                println!("\u{1b}[93m\"{}\": \"{}\"\u{1b}[0m", "WARNING","--configfile value not found in args!  Will use default configfile." );
            }
        }
        if argument == "--help" {
            println!("\nusage: ./target/debug/rust --configfile <filepath>\n");
            process::exit(0x0100);
        }
        i += 1
    }
    let config = HashMap::from([
        ("filename", filename),
    ]);
    return config
}

// read file; display and return contents
pub fn read_file(filename: &str) -> String {
    println!("Read file: {}", &filename);
    let contents = fs::read_to_string(&filename)
        .expect("\u{1b}[31m\"Could not read file\u{1b}[39m");
    return contents;
}
