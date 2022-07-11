mod support;
use std::env;

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let args_v: Vec<String> = env::args().collect();
    let config = support::analyze_args(&args_v);
    
    match config.get(&"filename") {
        Some(&filename) => println!("\nFile content...\n{}", support::read_file(filename)),
        _ => println!("No filename."),
    }
}
