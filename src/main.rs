use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Query file name from command line arguments
    let query = &args[1];
    let filename = &args[2];

    // Print the query and filename
    println!("searching for: {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("File contents:\n{}", contents);
}
