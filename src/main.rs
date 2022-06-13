use std::{env, process};
use alpha_to_bin::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = alpha_to_bin::run(config) {
        println!("Application failed: {}", e);
        process::exit(1);
    }
}
