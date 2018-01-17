extern crate minigrep;

use std::process;

use minigrep::Args;

fn main() {
    let args = Args::new().unwrap_or_else(|err| {
        println!("Problems parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
