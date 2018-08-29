use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
extern crate time;
use time::PreciseTime;
use std::collections::HashSet;
use std::cmp::min;

// Linux-only :angry: terminal imports, to make it look <<<nice>>> in-terminal

// extern crate termcolor;
// extern crate spinners;
// use spinners::{Spinner, Spinners};
// use std::thread::sleep;
// use std::io::Write;
// use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
// use std::time::Duration;

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

// Converts word file (containing all words in the English language) into a HashSet
pub fn assemble_word_hashset<'a>(contents: &'a str) -> HashSet<&'a str> {

    let mut word_set = HashSet::new();

    for (i, line) in contents.lines().enumerate() {
		if i >= 45 {
		
			let split_line = line.split(" ");
			let vec = split_line.collect::<Vec<&str>>();

			for item in vec {
				let item = item.trim_matches('\\');            
				word_set.insert(item);
			};
		}
    }

    word_set
}

fn main() {
    let start = PreciseTime::now();
    
    // Reading in command-line args, collecting them into a vector.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Checking file {}", config.filename);

    // Linux-only terminal spinners. :pensive:

    // let sp = Spinner::new(Spinners::Dots9, "Waiting for 3 seconds".into());
    // sleep(Duration::from_secs(3));
    // sp.stop();

    // Reading in the file the user wishes to spell-check.
    let mut f = File::open(config.filename).expect("File Not Found :(");

    let mut contents = String::new();
    &f.read_to_string(&mut contents)
    .expect("Something went wrong :( Could not read the file");

    // Reading in the words.txt file that contains all words in the English language (except brand names, etc.,)
    let mut word_file_contents = String::new();
    let mut word_file = File::open("words.txt").expect("File Not Found :(");
    &word_file.read_to_string(&mut word_file_contents)
    .expect("Something went wrong :( Could not read the file");

    // Reading in the cn_words.txt file that contains punctuation.
    let mut cn_file_contents = String::new();
    let mut cn_word_file = File::open("cn_words.txt").expect("File Not Found :(");
    &cn_word_file.read_to_string(&mut cn_file_contents)
    .expect("Something went wrong :( Could not read the file");
	
	// Reading in the suggestion_words.txt that contains a smaller list of words used for suggestions 
	let mut suggestion_words_contents = String::new();
	let mut suggestion_words_file = File::open("suggestion_words.txt").expect("File Not Found :(");
	&suggestion_words_file.read_to_string(&mut suggestion_words_contents)
	.expect("Something went wrong :( Could not read the file");

    let word_hashset = assemble_word_hashset(&word_file_contents);
    let cn_word_hashset = assemble_word_hashset(&cn_file_contents);
	let suggestion_words_hashset = assemble_word_hashset(&suggestion_words_contents);

    search(&contents, word_hashset, cn_word_hashset, suggestion_words_hashset);

    let end = PreciseTime::now();
	let time_taken = format!("{}", start.to(end));
	let time_taken = &time_taken[2..];
    println!("Took {} seconds to spell-check.", time_taken);
}

pub fn search<'a>(contents: &'a str, word_hashset :  HashSet<&'a str>, cn_word_hashset : HashSet<&'a str>, 
					suggestion_words_hashset: HashSet<&'a str>) {
    let mut line_number = 0;
    let mut total_spelling_errors = 0;
    let mut word_count = 0;

    for line in contents.lines() {
        line_number += 1;
        let split_line = line.split(" ");
        let vec = split_line.collect::<Vec<&str>>();

        for item in &vec {
            word_count += 1;
            let item = item.to_lowercase();
            let item_str = item.as_str();
            
            if cn_word_hashset.contains(item_str) {
                continue;
            }
            else {

                let mut stripped_word = String::new();
                
                for c in item_str.chars() {
                    if c.is_alphabetic() {
                        stripped_word.push(c);
                    }
                    else {
                        continue
                    }
                }

                let str_stripped_word : &str = &stripped_word;

                if word_hashset.contains(&str_stripped_word) {
                    continue;
                }
                else {
                    // Spelling mistake or else punctuation needs to be stripped out
                    total_spelling_errors += 1;
                    // let mut stdout = StandardStream::stdout(ColorChoice::Auto);
                    // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                    // writeln!(&mut stdout, "LINE {}, Spelling error: {} ", line_number, str_stripped_word);

					println!("Line {}: {}", line_number, line);
					println!("Spelling error: {}.", str_stripped_word);
						
					let mut replacements = Vec::new();
					
                    for word in &suggestion_words_hashset {
                        if edit_distance(&word.to_string(), &str_stripped_word.to_string()) <= 1 {
                            replacements.push(word);
                        }
                    }

                    if replacements.len() > 0 {
                        println!("Suggestions: ");
                        for (i, replacement) in replacements.iter().enumerate() {
						    println!("{}. {}", i, replacement);
					    }
                    }
                }
            }
        }
    }
    
    println!("Total errors: {}", total_spelling_errors);
    println!("Go over these errors, some may have been flagged inappropriately.");
    println!("Word count: {}", word_count);
}

pub fn edit_distance<'a, 'b>(s1: &'a String, s2: &'b String) -> u32 {
    let rows = s2.chars().count() + 1;
    let columns = s1.chars().count() + 1;

    let mut matrix = Vec::new();

    for _ in 0..rows {
        matrix.push(Vec::new());
    }
    
    for mut row in &mut matrix {
        for _ in 0..columns {
            row.push(0);
        }
    }

    for num in 0..columns {
        matrix[0][num] = num;
    }

    for num in 0..rows {
        matrix[num][0] = num;
    }

    for i in 1..rows {
        for j in 1..columns {
            if s2[i-1..i] == s1[j-1..j] {
                matrix[i][j] = matrix[i-1][j-1];
            }
            else {
                matrix[i][j] = 1 + min(matrix[i-1][j-1], min(matrix[i-1][j], matrix[i][j-1]));
            }
        }
    }

    matrix[rows-1][columns-1] as u32
}
