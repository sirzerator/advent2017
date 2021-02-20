extern crate regex;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Program {
    name: String,
    weight: u32,
    supports: Vec<String>,
    total_weight: u32,
}

impl Program {
    fn sum_tower(&mut self, programs: HashMap<String, &Program>) -> u32 {
        for supported in self.supports.iter() {
            match programs.get(supported) {
                None => continue,
                Some(program) => self.total_weight += program.sum_tower(programs)
            };
        }

        self.total_weight
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input filename.");
        println!("Example: ./day7 input7.txt");
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
    let mut programs: Vec<Program> = Vec::new();
    let mut programs_supported: HashMap<String, bool> = HashMap::new();
    let mut programs_map: HashMap<String, &Program> = HashMap::new();

    for line in buffer.lines() {
        match line {
            Err(reason) => println!("{}", reason.description()),
            Ok(line) => {
                let program = parse_line(line);
                programs.push(program);
            }
        }
    }

    for program in programs.iter() {
        for supported in program.supports.iter() {
            programs_supported.insert(supported.to_string(), true);
        }

        if !programs_supported.contains_key(&program.name) {
            programs_supported.insert(program.name.to_string(), false);
        }

        programs_map.insert(program.name.to_string(), program);
    }

    let mut root: String = String::new();
    for (name, supported) in programs_supported.iter() {
        if !supported {
            root = name.to_string();
            println!("{} is not supported.", name);
        }
    }

    match programs_map.get(&root) {
        None => (),
        Some(root_program) => println!("{}", root_program.sum_tower(programs_map))
    };

    println!("{}", "end")
}

fn parse_line(line: String) -> Program {
    let re = Regex::new(r"^(?P<n>\w+)\s\((?P<w>\d+)\)(\s->\s)?(?P<p>.+)?").unwrap();
    let mut name: String = String::new();
    let mut supports: Vec<String> = Vec::new();
    let mut weight: u32 = 0;

    for cap in re.captures_iter(&line) {
        match &cap.name("n") {
            &Some(n) => name = n.as_str().to_string(),
            &None => continue
        };
        match &cap.name("w") {
            &None => continue,
            &Some(wth) => match wth.as_str().parse::<u32>() {
                Err(_) => {
                    println!("Invalid weight.");
                    ::std::process::exit(3);
                },
                Ok(w) => weight = w
            }
        };
        match &cap.name("p") {
            &None => continue,
            &Some(p) => for program in p.as_str().split_whitespace() {
                let re = Regex::new(r",").unwrap();
                supports.push(re.replace_all(program, "").to_string());
            }
        };
    }

    let mut new_program = Program {
        name: name,
        weight: weight,
        supports: supports,
        total_weight: weight
    };

    println!("{:?}", new_program);

    new_program
}
