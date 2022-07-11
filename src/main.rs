mod support;
use std::env;

fn main() {
    let args_v: Vec<String> = env::args().collect();
    support::analyze_args(&args_v);
}
