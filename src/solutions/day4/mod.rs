use std::fs;

pub fn solve() {
    match fs::read_to_string("./input/day4/input") {
        Ok(content) => {
            let mut cnt = 0;
            for line in content.split("\n") {
                if let Some((left, right)) = line.trim().split_once(",") {
                    let (a1, b1) = left.split_once("-").unwrap();
                    let (a2, b2) = right.split_once("-").unwrap();


                    if (a1.parse::<i32>().unwrap() <= a2.parse().unwrap() &&
                        b1.parse::<i32>().unwrap()  >= b2.parse().unwrap()) 
                    || 
                        (a2.parse::<i32>().unwrap() <= a1.parse().unwrap() &&
                    b2.parse::<i32>().unwrap() >= b1.parse().unwrap()) {
                        cnt += 1;
                    }
                } else {
                    // ending line
                }
            }
            println!("total: {}", cnt);
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}
