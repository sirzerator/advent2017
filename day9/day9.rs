use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input string.");
        println!("Example: ./day9 $(cat input9.txt)");
        ::std::process::exit(1);
    }

    let input_string: &String = &args[1];

    let mut opened_groups: u32 = 0;
    let mut ignore: bool = false;
    let mut ignore_next: bool = false;
    let mut garbage: u32 = 0;

    let mut score: u32 = 0;

    for ch in input_string.chars() {
        //println!("{}", ch);
        //println!("{} {} {}", opened_groups, ignore, ignore_next);
        if ignore_next {
            ignore_next = false;
            continue;
        }

        if ch == '!' {
            ignore_next = true;
            continue;
        }

        if ignore && ch != '>' {
            garbage += 1;
            continue;
        }

        if ch == '<' {
            ignore = true;
        } else if ch == '>' {
            ignore = false;
        } else if ch == '{' {
            opened_groups += 1;
        } else if ch == '}' {
            score += opened_groups;
            opened_groups -= 1;
        }
    }

    println!("String has a score of {}.", score);
    println!("String contains {} garbage characters.", garbage);
}
