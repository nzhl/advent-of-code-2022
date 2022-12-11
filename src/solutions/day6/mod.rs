use std::fs;
use std::collections::hash_map::HashMap;

pub fn has_duplicate_char(snippet: &str) -> bool {
    let mut map = HashMap::new();
    for each in snippet.chars() {
       if let Some(_) = map.get(&each) {
            return true;
        }
        map.insert(each, 1);
    }
    return false;
}

pub fn part1() {
    match fs::read_to_string("./input/day6/input") {
        Ok(content) => {
            for line in content.split("\n") {
                if line.trim().len() != 0 {
                    let mut left = 0;
                    let mut right = 4;

                    loop {
                        if !has_duplicate_char(&line[left..right]) {
                            break;
                        }
                        left = left + 1;
                        right = right + 1;
                    }
                    println!("result: {}", right);
                } else {
                    // skip
                }
            }

        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }

}

pub fn part2() {
    match fs::read_to_string("./input/day6/input") {
        Ok(content) => {
            for line in content.split("\n") {
                if line.trim().len() != 0 {
                    let mut left = 0;
                    let mut right = 14;

                    loop {
                        if !has_duplicate_char(&line[left..right]) {
                            break;
                        }
                        left = left + 1;
                        right = right + 1;
                    }
                    println!("result: {}", right);
                } else {
                    // skip
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
