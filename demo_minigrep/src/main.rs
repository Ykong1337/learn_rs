use std::{env, process};
use demo_minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with config: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}

