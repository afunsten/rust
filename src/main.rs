mod support;

use std::env;


fn main() {
    //defaults

    let args_v: Vec<String> = env::args().collect();
    support::analyze_args(&args_v);
    //let args = env::args();

}
