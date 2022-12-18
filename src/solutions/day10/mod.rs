use std::fs;

pub fn part1() {
    match fs::read_to_string("./input/day10/input") {
        Ok(content) => {
            let mut sum = 0;
            let mut cur_cycle = 0;
            let mut cur_value = 1;
            let mut target_cycles = vec![220, 180, 140, 100, 60, 20];

            for maybe_value in content.split("\n").filter_map(|each|{
                if each.len() == 0 {
                    None
                } else if each == "noop" {
                    Some(None)
                } else {
                    let value = each[5..].parse::<i32>().unwrap();
                    Some(Some(value))
                }
            }) {
                let target = *target_cycles.last().unwrap();
                if let Some(value) = maybe_value {
                    if cur_cycle < target && cur_cycle + 2 >= target {
                        sum += target * cur_value;
                        target_cycles.pop();
                        if target_cycles.len() == 0 {
                            break;
                        }
                    }
                    cur_value += value;
                    cur_cycle += 2;
                } else {
                    cur_cycle += 1;
                    if cur_cycle == target {
                        sum += target * cur_value;
                        target_cycles.pop();
                        if target_cycles.len() == 0 {
                            break;
                        }
                    }
                }
            }

            println!("Sum: {}", sum);
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}

const SCREEN_WIDTH: i32 = 40;
fn crt_print (cur_cycle: i32, cur_value: i32) {
    let print_pos = (cur_cycle - 1) % SCREEN_WIDTH;
    if cur_value - 1 <= print_pos && cur_value + 1 >= print_pos {
        print!("#");
    } else {
        print!(" ");
    }
    if print_pos + 1 == 40 {
        println!("");
    }
}

pub fn part2() {
    match fs::read_to_string("./input/day10/input") {
        Ok(content) => {
            let mut cur_cycle = 0;
            let mut cur_value = 1;

            for maybe_value in content.split("\n").filter_map(|each|{
                if each.len() == 0 {
                    None
                } else if each == "noop" {
                    Some(None)
                } else {
                    let value = each[5..].parse::<i32>().unwrap();
                    Some(Some(value))
                }
            }) {
                if let Some(value) = maybe_value {
                    crt_print(cur_cycle + 1, cur_value);               
                    crt_print(cur_cycle + 2, cur_value);               
                    cur_value += value;
                    cur_cycle += 2;
                } else {
                    cur_cycle += 1;
                    crt_print(cur_cycle, cur_value);               
                }
            }
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}

pub fn solve() {
    part1();
    part2();
}
