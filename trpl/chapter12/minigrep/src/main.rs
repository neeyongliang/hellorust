use minigrep::Config;
use std::process;
use std::env;

// Usage: cargo run WORD FILE
// cargo run rust why-rust.txt
// CASE_INSENSITIVE=1 cargo run rust why-rust.txt

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

