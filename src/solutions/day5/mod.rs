use std::fs;
use std::vec::Vec;
use regex::Regex;

pub fn part1() {
    let mut stacks = vec![
        vec!["B", "S", "V", "Z", "G", "P", "W"],
        vec!["J", "V", "B", "C", "Z", "F"],
        vec!["V", "L", "M", "H", "N", "Z", "D", "C"],
        vec!["L", "D", "M", "Z", "P", "F", "J", "B"],
        vec!["V", "F", "C", "G", "J", "B", "Q", "H"],
        vec!["G", "F", "Q", "T", "S", "L", "B"],
        vec!["L", "G", "C", "Z", "V"],
        vec!["N", "L", "G"],
        vec!["J", "F", "H", "C"],
    ];
    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    match fs::read_to_string("./input/day5/input") {
        Ok(content) => {

            for line in content.split("\n") {
                if line.trim().starts_with("move") {
                    let captures = re.captures_iter(line).next().unwrap();
                    let amount = captures[1].parse::<usize>().unwrap();
                    let src_stack = captures[2].parse::<usize>().unwrap() - 1;
                    let dst_stack = captures[3].parse::<usize>().unwrap() - 1;

                    for _ in 0..amount {
                        let src = stacks.get_mut(src_stack).unwrap();
                        let create = (*src).pop();
                        if let Some(create) = create {
                            let dst = stacks.get_mut(dst_stack).unwrap();
                            dst.push(create);
                        }
                    }
                } else {
                    // skip
                }
            }

            let result = stacks.iter().filter_map(|stack|stack.last().map(|&v|v)).collect::<Vec<&str>>();
            println!("result {:#?}", result.join(""));
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }

}
pub fn part2() {
    let mut stacks = vec![
        vec!["B", "S", "V", "Z", "G", "P", "W"],
        vec!["J", "V", "B", "C", "Z", "F"],
        vec!["V", "L", "M", "H", "N", "Z", "D", "C"],
        vec!["L", "D", "M", "Z", "P", "F", "J", "B"],
        vec!["V", "F", "C", "G", "J", "B", "Q", "H"],
        vec!["G", "F", "Q", "T", "S", "L", "B"],
        vec!["L", "G", "C", "Z", "V"],
        vec!["N", "L", "G"],
        vec!["J", "F", "H", "C"],
    ];
    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    match fs::read_to_string("./input/day5/input") {
        Ok(content) => {
            for line in content.split("\n") {
                if line.trim().starts_with("move") {
                    let captures = re.captures_iter(line).next().unwrap();
                    let amount = captures[1].parse::<usize>().unwrap();
                    let src_stack = captures[2].parse::<usize>().unwrap() - 1;
                    let dst_stack = captures[3].parse::<usize>().unwrap() - 1;

                    let src = stacks.get_mut(src_stack).unwrap();
                    let mut vecs = src.drain(src.len()-amount..src.len()).as_slice().to_vec();
                    let dst = stacks.get_mut(dst_stack).unwrap();
                    dst.append(&mut vecs);
                } else {
                    // skip
                }
            }

            let result = stacks.iter().filter_map(|stack|stack.last().map(|&v|v)).collect::<Vec<&str>>();
            println!("result {:#?}", result.join(""));
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
