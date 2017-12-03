use std::env;

fn main() {
    const RADIX: u32 = 10;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input string representing an integer.");
        println!("Example: ./day1 $(cat input1.txt)");
        ::std::process::exit(1);
    }

    let input_string: &String = &args[1];

    let mut vec: Vec<u32> = Vec::new();
    for ch in input_string.chars() {
        match ch.to_digit(RADIX) {
            None => {
                println!("The input string contains non numeric characters.");
                ::std::process::exit(2)
            },
            Some(i) => vec.push(i)
        }
    }

    let mut total: u32 = 0;
    let mut alt_total: u32 = 0;

    let length = vec.len();
    let half_length = length / 2;

    for index in 0..length {
        if vec[index] == vec[(index + 1) % length] {
            total += vec[index]
        }

        if vec[index] == vec[(index + half_length) % length] {
            alt_total += vec[index]
        }
    }

    println!("Next element sum : {} .", total);
    println!("Halfway through element sum : {} .", alt_total)
}
