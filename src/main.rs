use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::mem;

struct Config {
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.\nYou must specify a filename to check.");
        }
        else if args.len() > 2 {
            return Err("Can only check one file at a time.")
        }
        let filename = args[1].clone();
        Ok(Config {filename})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("In file {}", config.filename);

    // Reading in a file
    let mut f = File::open(config.filename).expect("File Not Found :(");

    let mut contents = String::new();
    &f.read_to_string(&mut contents)
    .expect("Something went wrong :( Could not read the file");

    let mut word_file_contents = String::new();
    let mut word_file = File::open("words.txt").expect("File Not Found :(");
    &word_file.read_to_string(&mut word_file_contents)
    .expect("Something went wrong :( Could not read the file");

    let word_vec = assemble_word_vec(&word_file_contents);

    for line in search(&contents, &word_vec) {
        println!("{}", line);
    }

    // println!("Contains:\n{}", contents);
}

pub fn search<'a>(contents: &'a str, word_vec : &Vec<&str>) -> Vec<&'a str> {
    let dict = vec!["dreary", "Who", "how", "somebody"];

    let mut results = Vec::new();

    for line in contents.lines() {
        let split_line = line.split(" ");
        let vec = split_line.collect::<Vec<&str>>();

        for item in &vec {
            // TODO strip out chars

            let mut stripped_word = String::new();

            
            let item = item.to_lowercase();
            let item_str = item.as_str();

            if word_vec.contains(&item_str) {
                continue;
            }
            else {
                // Spelling mistake or else punctuation needs to be stripped out

                for c in item.chars() {
                if c.is_alphabetic() {
                        stripped_word.push(c);
                }
                else {
                    continue
                }
            }
            let str_stripped_word : &str = &stripped_word;
                if word_vec.contains(&str_stripped_word) {
                continue;
            }
            else {
                println!("Spelling error!: {}", str_stripped_word);
            }
            }
        }
    }
    results
}

// fn strip_punc(item : &str) -> str {
//     let mut stripped_word = String::new();

//     for c in item.chars() {
//         if c.is_alphabetic() {
//             stripped_word.push(c);
//         }
//         else {
//             continue
//         }
//     }
//     stripped_word 
// }

pub fn assemble_word_vec<'a>(contents: &'a str) -> Vec<&'a str> {

    let mut word_vec = vec!["hi"];

    for line in contents.lines() {
        let split_line = line.split(" ");
        let vec = split_line.collect::<Vec<&str>>();

        for item in &vec {
            word_vec.push(item);
        };
    }

    word_vec
}

