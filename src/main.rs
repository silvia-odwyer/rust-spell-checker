use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
extern crate time;
use time::PreciseTime;
use std::collections::HashSet;
use std::cmp::min;

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

pub fn assemble_suggestion_hashset<'a>(contents: &'a str) -> HashSet<&'a str> {
	let mut word_set = HashSet::new();

    for line in contents.lines() {	
		word_set.insert(line);
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
	
	// Reading in the suggestion_words.txt that contains a smaller list of words used for suggestions 
	let mut ranked_words_contents = String::new();
	let mut ranked_words_file = File::open("words_ranked.txt").expect("File Not Found :(");
	&ranked_words_file.read_to_string(&mut ranked_words_contents)
	.expect("Something went wrong :( Could not read the file");

    let word_hashset = assemble_word_hashset(&word_file_contents);
	let ranked_words_hashset = assemble_suggestion_hashset(&ranked_words_contents);

    search(contents, word_hashset, ranked_words_hashset);

    let end = PreciseTime::now();
	let time_taken = format!("{}", start.to(end));
	let time_taken = &time_taken[2..];
    println!("Took {} seconds to spell-check.", time_taken);
}

pub fn search<'a>(contents: String, word_hashset : HashSet<&'a str>, ranked_words_hashset: HashSet<&'a str>) {
    let mut total_spelling_errors = 0;
    let mut word_count = 0;
    
    // Strip non-full-stop punctuation and store sentences in a vec.
    let modified_contents = strip_punc(&contents);
    let modified_contents = modified_contents.split(".").collect::<Vec<&str>>();    

    let mut sentences_and_questions = Vec::new();

    // Strip out empty items in the vec.
    for sentence in modified_contents {
        if !sentence.is_empty() {
            sentences_and_questions.push(sentence.trim());
        }
    }

    for sentence in &sentences_and_questions {
        let split_sentence = sentence.split(" ");
        let vec = split_sentence.collect::<Vec<&str>>();
        
        let mut current_word_index = 0;

        // Iterate over every word in the sentence 
        for word in &vec {
            current_word_index += 1;
            word_count += 1;

            if current_word_index == 1 {

                let mut chars = word.chars();
                
                if !word.is_empty() {
                    let first_char = chars.next().expect("No first character in first word.");

                    // If the word's first char is uppercase, conv to lowercase.
                    if first_char.is_uppercase() {
                        if word_hashset.contains(word.to_lowercase().as_str()) || word_hashset.contains(word) {
                            continue;
                        } else {
                            total_spelling_errors += 1;
                            println!("Spelling error: {}.", word);
                            give_suggestions(&ranked_words_hashset, &word);
                            continue;
                        }
                    }
                    else {
                        total_spelling_errors += 1;
                        println!("Spelling error: {}.", word);
                        println!("Suggestion: Capitalize {}. ", first_char);
                        continue;
                    }
                }
            }
                        
            if word_hashset.contains(word) {
                continue;
            }

            else {
                // Spelling mistake or else punctuation needs to be stripped out
                total_spelling_errors += 1;
				println!("Spelling error: {}.", word);
				give_suggestions(&ranked_words_hashset, &word);
            }
        }
    }   
    
    println!("Total errors: {}", total_spelling_errors);
    println!("Go over these errors, some may have been flagged inappropriately.");
    println!("Word count: {}", word_count);
}

// Strip extraneous punctuation that isn't a full stop
pub fn strip_punc(contents: &str) -> String {
    let mut modified_contents = convert_to_sentences(&contents);

    let punc = ["?", "!", ",", ";", ":", "[", "]", "(", ")", "{", "}", "\""];

    for i in 0..punc.len() {
        let symbol = punc[i];
        modified_contents = modified_contents.replace(symbol, "");
    }

    return modified_contents
}

// Convert question marks, exclamation marks and other sentence-ending 
// delimiters into full stops
pub fn convert_to_sentences(contents: &str) -> String {
    let mut modified_contents = contents.replace("?", ".");
    modified_contents = modified_contents.replace("!", ".");
    return modified_contents
}

pub fn give_suggestions<'a>(ranked_words_hashset: &HashSet<&'a str>, str_stripped_word: &'a str) {
    // Spelling mistake or else punctuation needs to be stripped out
    			
	let mut replacements = Vec::new();
	let mut replacements_distance_is_two = Vec::new();
					
	for word in ranked_words_hashset {
		let word_and_rank: Vec<&str> = word.split(" ").collect();
		let edit_distance = edit_distance(&word_and_rank[0].to_string(), &str_stripped_word.to_string());
						
		if edit_distance <= 1 {
			replacements.push(word_and_rank);
		}
		else if edit_distance <= 2 {
			replacements_distance_is_two.push(word_and_rank);
		}
	}
					
	let mut ranks_dist_is_one = Vec::new();
	for replacement in &replacements {
		ranks_dist_is_one.push(replacement[1]);
	}
				
 	ranks_dist_is_one.sort();
 	ranks_dist_is_one.truncate(3);
					
 	let mut popular_words_dist_is_one = Vec::new(); 
					
	for rank in ranks_dist_is_one {
		for replacement in &replacements {
			if replacement[1] == rank {
				popular_words_dist_is_one.push(replacement[0]);
			}
		}
	}
					
 	let mut ranks_dist_is_two = Vec::new();
	for replacement in &replacements_distance_is_two {
		ranks_dist_is_two.push(replacement[1]);
	}
					
	ranks_dist_is_two.sort();
	ranks_dist_is_two.truncate(2);
					
	let mut popular_words_dist_is_two = Vec::new();
					
	for rank in ranks_dist_is_two {
		for replacement in &replacements_distance_is_two {
			if replacement[1] == rank {
	    		popular_words_dist_is_two.push(replacement[0]);
			}
		}
	}
					
	let mut final_replacements = popular_words_dist_is_one;
					
	if popular_words_dist_is_two.len() > 0 {
		for replacement in popular_words_dist_is_two.iter() {
			final_replacements.push(replacement);
		}
	}
					
	if final_replacements.len() > 0 {
		println!("Suggestions: ");
						
		for (i, replacement) in final_replacements.iter().enumerate() {
			println!("{}. {}", i, replacement);
		}
	}
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
