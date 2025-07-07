use std::env;
use std::error::Error;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config= Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        exit(1);
    });

    // Print the query and filename
    println!("searching for: {}", config.query);
    println!("In file {}", config.filename);

   if let Err(e) = run(config) {
        println!("Application error: {}", e);
        exit(1);
    
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    println!("File contents:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided. Usage: <query> <filename>");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename})

    }
    
}

