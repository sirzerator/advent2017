extern crate regex;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input filename.");
        println!("Example: ./day8 input8.txt");
        ::std::process::exit(1);
    }

    let path: &Path = Path::new(&args[1]);
    let file = match File::open(&path) {
        Err(reason) => {
            let display = path.display();
            println!(
                "Couldn't open {}: {}",
                display,
                reason.description());
            ::std::process::exit(2)
        },
        Ok(file) => file,
    };


    let buffer = BufReader::new(&file);
    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut largest_value_ever: i32 = 0;
    for line in buffer.lines() {
        match line {
            Err(reason) => println!("{}", reason.description()),
            Ok(line) => {
                let re = Regex::new(r"(\w+)\s(dec|inc)\s(-?\d+)\sif\s(\w+)\s(<=|==|!=|>=|>|<)\s(-?\d+)").unwrap();
                for cap in re.captures_iter(&line) {
                    if !registers.contains_key(&cap[1]) {
                        registers.insert(cap[1].to_string(), 0);
                    }
                    if !registers.contains_key(&cap[4]) {
                        registers.insert(cap[4].to_string(), 0);
                    }

                    let inc_value: i32 = match cap[3].parse::<i32>() {
                        Err(_) => {
                            println!("Contains invalid integer.");
                            ::std::process::exit(3)
                        },
                        Ok(c) => c
                    };
                    let comp_value: i32 = match cap[6].parse::<i32>() {
                        Err(_) => {
                            println!("Contains invalid integer.");
                            ::std::process::exit(3)
                        },
                        Ok(c) => c
                    };
                    let register_value: i32 = registers[&cap[4]];

                    let mut matches: bool = false;
                    if ">" == &cap[5] {
                        matches = register_value > comp_value;
                    } else if "<" == &cap[5] {
                        matches = register_value < comp_value;
                    } else if ">=" == &cap[5] {
                        matches = register_value >= comp_value;
                    } else if "<=" == &cap[5] {
                        matches = register_value <= comp_value;
                    } else if "==" == &cap[5] {
                        matches = register_value == comp_value;
                    } else if "!=" == &cap[5] {
                        matches = register_value != comp_value;
                    }

                    if matches {
                        let previous_value: i32 = registers[&cap[1]];
                        if &cap[2] == "inc" {
                            let new_value = previous_value + inc_value;
                            if new_value > largest_value_ever {
                                largest_value_ever = new_value;
                            }

                            registers.insert(
                                cap[1].to_string(),
                                previous_value + inc_value
                            );
                        } else {
                            let new_value = previous_value - inc_value;
                            if new_value > largest_value_ever {
                                largest_value_ever = new_value;
                            }
                            registers.insert(
                                cap[1].to_string(),
                                previous_value - inc_value
                            );
                        }
                    }
                }
            }
        }
    }

    let mut largest_value: i32 = 0;
    let mut register: String = String::new();
    for (key, value) in registers.iter() {
        if *value > largest_value {
            largest_value = *value;
            register = key.to_string();
        }
    }
    println!("Largest value in register {} is {}.", register, largest_value);
    println!("Largest value ever is {}.", largest_value_ever);
}
