use std::env;
use std::process::exit;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config= Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        exit(1);
    });

    // Print the query and filename
    println!("searching for: {}", config.query);
    println!("In file {}", config.filename);

   if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        exit(1);
    
    }
}

