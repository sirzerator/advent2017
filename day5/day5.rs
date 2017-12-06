use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

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
    let mut offsets: Vec<i32> = Vec::new();
    let mut alt_offsets: Vec<i32> = Vec::new();

    for line in buffer.lines() {
        match line {
            Err(reason) => println!("{}", reason.description()),
            Ok(line) => {
                match line.parse::<i32>() {
                    Err(_) => {
                        println!("An input offset is not a number.");
                        ::std::process::exit(3)
                    },
                    Ok(i) =>{
                        offsets.push(i);
                        alt_offsets.push(i);
                    },
                }
            }
        }
    }

    let exit_index: i32 = offsets.len() as i32;

    let mut current_index: i32 = 0;
    let mut steps: i32 = 0;

    while current_index < exit_index {
        let offset_at_index: i32 = offsets[current_index as usize];
        offsets[current_index as usize] += 1;
        current_index += offset_at_index;
        steps += 1;
    }

    println!("It takes {} steps with the first rule.", steps);

    let mut alt_current_index: i32 = 0;
    let mut alt_last_index: i32 = 0;
    let mut alt_steps: i32 = 0;

    while alt_current_index < exit_index {
        let offset_at_index: i32 = alt_offsets[alt_current_index as usize];
        if offset_at_index >= 3 {
            alt_offsets[alt_current_index as usize] -= 1;
        } else {
            alt_offsets[alt_current_index as usize] += 1;
        }

        alt_current_index += offset_at_index;
        alt_steps += 1;
    }

    println!("It takes {} steps with the second set of rules.", alt_steps)
}
