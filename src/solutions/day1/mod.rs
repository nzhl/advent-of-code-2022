use std::fs;
use std::vec::Vec;

pub fn solve() {
    match fs::read_to_string("./input/day1/input") {
        Ok(content) => {
            let mut max = 0;
            let mut cur = 0;

            let mut max3 = Vec::new();
            for line in content.split("\n") {
                if line.trim().len() == 0 {
                    if cur > max {
                        max = cur;
                    }

                    max3.push(cur);
                    if max3.len() > 3 {
                        let min = max3.iter().min().unwrap().to_owned();
                        max3 = max3
                            .iter()
                            .filter_map(|&x| if x != min {Some(x)} else {None})
                            .collect::<Vec<u32>>();
                        while max3.len() < 3 {
                            max3.push(min);
                        }
                    }

                    cur = 0;


                } else {
                    cur += line.parse::<u32>().unwrap();
                }
            }

            println!("Max: {}", max);
            println!("Sum: {}", max3.iter().sum::<u32>());
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}
