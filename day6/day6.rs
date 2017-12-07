use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input string representing a sequence of integers.");
        println!("Example: ./day6 $(cat input6.txt)");
        ::std::process::exit(1);
    }

    let mut blocks: Vec<u32> = Vec::new();
    for i in 1..args.len() {
        match args[i].parse::<u32>() {
            Err(_) => {
                println!("The input string contains non numeric characters.");
                ::std::process::exit(2)
            },
            Ok(num) => blocks.push(num)
        }
    }

    let mut steps: u32 = 0;
    let blocks_length: usize = blocks.len();
    let mut seen: HashMap<String, bool> = HashMap::new();
    let mut last_seen: String;

    loop {
        let blocks_string: String = str_representation(&blocks);
        if seen.contains_key(&blocks_string) {
            last_seen = blocks_string.to_string();
            break;
        }
        seen.insert(blocks_string, true);

        let mut index: usize = find_index_biggest(&blocks);
        let mut at_index: u32 = blocks[index];
        blocks[index] = 0;

        while at_index > 0 {
            index += 1;
            index %= blocks_length;
            blocks[index] += 1;
            at_index -= 1
        };

        steps += 1;
    }

    let mut alt_steps: u32 = 0;

    loop {
        let mut index: usize = find_index_biggest(&blocks);
        let mut at_index: u32 = blocks[index];
        blocks[index] = 0;

        while at_index > 0 {
            index += 1;
            index %= blocks_length;
            blocks[index] += 1;
            at_index -= 1
        };

        alt_steps += 1;

        let blocks_string: String = str_representation(&blocks);
        if blocks_string == last_seen {
            break;
        }
    }

    println!("The sequence will loop after {} steps.", steps);
    println!("The sequence will reloop after {} steps.", alt_steps);
}

fn find_index_biggest(blocks: &Vec<u32>) -> usize {
    let nb_blocks: usize = blocks.len();

    let mut biggest_index: usize = 0;
    let mut at_biggest_index: u32 = 0;

    for i in 0..nb_blocks {
        if blocks[i] > at_biggest_index {
            biggest_index = i;
            at_biggest_index = blocks[i]
        }
    }

    biggest_index
}

fn str_representation(blocks: &Vec<u32>) -> String {
    blocks.iter()
        .map(|p| p.to_string())
        .fold(String::new(), |mut acc, p| {
            acc = format!("{}{}", acc, p);
            acc
        })
}
