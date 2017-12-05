use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::SeekFrom;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input filename.");
        println!("Example: ./day4 input4.txt");
        ::std::process::exit(1);
    }

    let path: &Path = Path::new(&args[1]);
    let display = path.display();

    let file = match File::open(&path) {
        Err(reason) => {
            println!(
                "Couldn't open {}: {}",
                display,
                reason.description());
            ::std::process::exit(2)
        },
        Ok(file) => file,
    };

    let buffer = BufReader::new(&file);
    let mut valid_passphrases: i32 = 0;

    'lines: for line in buffer.lines() {
        let mut passphrases: HashMap<String, bool> = HashMap::new();

        match line {
            Err(reason) => println!("{}", reason.description()),
            Ok(line) => {
                for value in line.split_whitespace() {
                    if passphrases.contains_key(value) {
                        continue 'lines;
                    }
                    passphrases.insert(value.to_string(), true);
                }

                valid_passphrases += 1;
            }
        }
    }

    let mut sec_buffer = BufReader::new(&file);
    match sec_buffer.seek(SeekFrom::Start(0)) {
        Ok(_) => (),
        Err(reason) =>  {
            println!(
                "Error during seek {}: {}",
                display,
                reason.description());
            ::std::process::exit(2)
        }
    }

    let mut valid_sorted_passphrases: i32 = 0;

    'sorted_lines: for line in sec_buffer.lines() {
        let mut sorted_passphrases: HashMap<String, bool> = HashMap::new();

        match line {
            Err(reason) => println!("{}", reason.description()),
            Ok(line) => {
                for value in line.split_whitespace() {
                    let mut chars: Vec<char> = value.chars().collect();
                    chars.sort_by(|a, b| b.cmp(a));

                    let sorted_value: String = chars.iter().cloned().collect::<String>();
                    if sorted_passphrases.contains_key(&sorted_value) {
                        continue 'sorted_lines;
                    }

                    sorted_passphrases.insert(sorted_value, true);
                }

                valid_sorted_passphrases += 1;
            }
        }
    }

    println!("{} passphrases are valid (no doubles).", valid_passphrases);
    println!("{} passphrases are valid (no anagrams).", valid_sorted_passphrases)
}
