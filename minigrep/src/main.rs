extern crate minigrep;

use std::process;

use minigrep::Args;

fn main() {
    let args = Args::new().unwrap_or_else(|err| {
        eprintln!("Problems parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
