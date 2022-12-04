use std::fs;

pub fn solve() {
    match fs::read_to_string("./input/day1/input") {
        Ok(content) => {
            let mut max = 0;
            let mut cur = 0;
            for line in content.split("\n") {
                if line.trim().len() == 0 {
                    if cur > max {
                        max = cur;
                    }
                    cur = 0;
                } else {
                    cur += line.parse::<u32>().unwrap();
                }
            }
            println!("Max: {}", max);
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}
