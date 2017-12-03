use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::path::Path;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input filename.");
        println!("Example: ./day2 input2.txt)");
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

    let mut total: i32 = 0;
    let mut alt_total: i32 = 0;

    for line in buffer.lines() {
        let mut values: Vec<i32> = Vec::new();

        match line {
            Err(reason) => println!("{}", reason.description()),
            Ok(line) => {
                for value in line.split_whitespace() {
                    match value.parse() {
                        Err(_) => {
                            println!("Invalid integer found.");
                            ::std::process::exit(2)
                        },
                        Ok(i) => values.push(i)
                    }
                }

                let mut max = std::i32::MIN;
                let mut min = std::i32::MAX;
                let mut multiples = 0;
                let length = values.len();

                for (idx, &int) in values.iter().enumerate() {
                    if int < min {
                        min = int;
                    }

                    if int > max {
                        max = int;
                    }

                    if multiples == 0 {
                        for pos in (idx + 1)..length {
                            if are_multiples(values[idx], values[pos]) {
                                multiples = factor(values[idx], values[pos]);
                                alt_total += multiples;
                                break
                            }
                        }
                    }
                }

                total += max - min;
            }
        };
    }

    println!("Max minus min checksum : {}", total);
    println!("Evenly divisible checksum : {}", alt_total)
}

fn are_multiples(val1: i32, val2: i32) -> bool {
    if val1 > val2 {
        return val1 % val2 == 0
    }

    return val2 % val1 == 0
}

fn factor(val1: i32, val2: i32) -> i32 {
    if val1 > val2 {
        return val1 / val2
    }

    val2 / val1
}
