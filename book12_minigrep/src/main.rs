use std::{env, process};
use book12_minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(228);
    });

    if let Err(e) = run(config) {
        eprintln!("Some error occured: {}", e);
    };
}