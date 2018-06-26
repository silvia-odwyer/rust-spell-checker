use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Reading in a file
    let mut f = File::open(config.filename).expect("File Not Found :(");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("Something went wrong :( Could not read the file");

    println!("Contains:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.\nYou must specify a filename or dir to zip.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}
