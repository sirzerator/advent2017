use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input number.");
        println!("Example: ./day3 $(cat input3.txt)");
        ::std::process::exit(1);
    }

    let input_string: &String = &args[1];

    let input_num: u32 = match input_string.parse::<u32>() {
        Err(_) => {
            println!("The input string is not a number.");
            ::std::process::exit(2)
        },
        Ok(i) => i
    };

    let mut num = input_num;

    let mut steps: Vec<char> = Vec::new();
    let mut ring: u32 = ring_of(num);

    while num > 1 {
        let lower_bound: u32 = sq(ring - 2) + 1;
        let upper_bound: u32 = sq(ring);

        // L
        if num == upper_bound || num < lower_bound + ring - 1 {
            steps.push('L');

            let new_num: u32 = left_index(num) as u32;
            if num == upper_bound {
                num -= 1;
            } else if num == lower_bound {
                num -= 1;
                ring -= 2;
            } else if num == lower_bound + ring - 2 {
                num += 1;
            } else {
                num = new_num;
                ring -= 2
            }
        // R
        } else if num > upper_bound - (2 * ring - 1) && num <= upper_bound - (ring - 1) {
            steps.push('R');

            let new_num: u32 = right_index(num) as u32;
            if num == upper_bound - (ring - 1) {
                num += 1;
            } else if num == upper_bound - (2 * ring - 1) + 1 {
                num -= 1;
            } else {
                num = new_num;
                ring -= 2;
            }
        // U
        } else if num > upper_bound - (ring - 1) {
            steps.push('U');

            num = up_index(num) as u32;
            ring -= 2;
        // D
        } else if num >= lower_bound + ring - 1 && num < lower_bound + 2 * ring - 2 {
            steps.push('D');

            num = down_index(num) as u32;
            ring -= 2;
        } else {
            println!("Unexpected position.");
            ::std::process::exit(3)
        }
    }

    println!("It takes {} steps to get from {} to 1.", steps.len(), input_string);
    println!("Full path is : {:?}", steps);

    let mut values: Vec<u32> = vec![1];
    let mut index: usize = 2;
    let mut next_value = 0;

    loop {
        next_value = 0;

        let up: usize = up_index(index as u32);
        let up_left: usize = left_index(up as u32);
        let up_right: usize = right_index(up as u32);
        let down: usize = down_index(index as u32);
        let down_left: usize = left_index(down as u32);
        let down_right: usize = right_index(down as u32);
        let left: usize = left_index(index as u32);
        let right: usize = right_index(index as u32);

        if up <= index {
            next_value += values[up - 1];
        }
        if up_left <= index {
            next_value += values[up_left - 1];
        }
        if up_right <= index {
            next_value += values[up_right - 1];
        }
        if down <= index {
            next_value += values[down - 1];
        }
        if down_left <= index {
            next_value += values[down_left - 1];
        }
        if down_right <= index {
            next_value += values[down_right - 1];
        }
        if left <= index {
            next_value += values[left - 1];
        }
        if right <= index {
            next_value += values[right - 1];
        }
        values.push(next_value);

        if (next_value > input_num) {
            break;
        }
        index += 1;
    }

    println!("Last value summed greater than input is : {}", next_value);
    println!("Values sequence is : {:?}", values);
}

fn sq(a: u32) -> u32 {
    a * a
}

fn ring_of(num: u32) -> u32 {
    let mut ring = 1;
    while sq(ring) < num {
        ring += 2
    };
    ring
}

fn up_index(index: u32) -> usize {
    let ring: u32 = ring_of(index);

    let upper_bound = sq(ring);
    let lower_bound = match upper_bound {
        1 => 1,
        _ => sq(ring - 2) + 1
    };

    if index == 1 {
        return 4;
    } else if index == upper_bound {
        return lower_bound as usize;
    } else if index < lower_bound + ring - 2 {
        return (index + 1) as usize;
    } else if index < lower_bound + 2 * (ring - 1) {
        let offset: u32 = index - lower_bound - (ring - 2);
        return (upper_bound + ring + 2 + offset) as usize;
    } else if index < lower_bound + 3 * (ring - 1) {
        return (index - 1) as usize;
    } else {
        let offset: u32 = upper_bound - index;
        return (lower_bound - offset) as usize;
    }
}

fn down_index(index: u32) -> usize {
    let ring: u32 = ring_of(index);

    let upper_bound = sq(ring);
    let lower_bound = match upper_bound {
        1 => 1,
        _ => sq(ring - 2) + 1
    };

    if index == 1 {
        return 8;
    } else if index == lower_bound {
        return upper_bound as usize;
    } else if index <= lower_bound + ring - 2 {
        return (index - 1) as usize;
    } else if index < lower_bound + (ring - 2) + (ring - 1) {
        let offset: u32 = index - (lower_bound + (ring - 1));
        return (sq(ring - 2) - 3 * (ring - 3) + offset) as usize;
    } else if index < upper_bound - (ring - 1) {
        return (index + 1) as usize;
    } else {
        let offset: u32 = ring - (upper_bound - index);
        return (sq(ring + 2) - (ring + 1) + offset) as usize;
    }
}

fn left_index(index: u32) -> usize {
    let ring: u32 = ring_of(index);

    let upper_bound = sq(ring);
    let lower_bound = match upper_bound {
        1 => 1,
        _ => sq(ring - 2) + 1
    };

    if index == 1 {
        return 6;
    } else if index == lower_bound {
        return (index - 1) as usize;
    } else if index < lower_bound + (ring - 2) {
        let offset: u32 = index - lower_bound;
        return (sq(ring - 4) + offset) as usize;
    } else if index <  lower_bound + (ring - 2) + (ring - 1) {
        return (index + 1) as usize;
    } else if index < upper_bound - (ring - 2) {
        let offset: u32 = upper_bound - (ring - 1 + index);
        return (sq(ring + 2) - (ring + 2) - offset) as usize;
    } else {
        return (index - 1) as usize;
    }
}

fn right_index(index: u32) -> usize {
    let ring: u32 = ring_of(index);

    let upper_bound = sq(ring);
    let lower_bound = match upper_bound {
        1 => 1,
        _ => sq(ring - 2) + 1
    };

    if index < lower_bound + (ring - 1) {
        let offset: u32 = index - lower_bound;
        return (upper_bound + offset + 2) as usize;
    } else if index < lower_bound + 2 * (ring - 1) {
        return (index - 1) as usize;
    } else if index < lower_bound + 3 * (ring - 1) - 1 {
        let offset: u32 = (upper_bound - ring) - index;
        return (lower_bound - (ring - 2) - offset) as usize;
    } else {
        return (index + 1) as usize;
    }
}
