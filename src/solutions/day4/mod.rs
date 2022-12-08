use std::fs;

pub fn solve() {
    match fs::read_to_string("./input/day4/input") {
        Ok(content) => {
            let mut cnt = 0;

            let mut cnt2 = 0;

            for line in content.split("\n") {
                if let Some((left, right)) = line.trim().split_once(",") {
                    let (a1, b1) = left.split_once("-").unwrap();
                    let (a2, b2) = right.split_once("-").unwrap();


                    let a1 = a1.parse::<i32>().unwrap();
                    let b1 = b1.parse::<i32>().unwrap();
                    let a2 = a2.parse::<i32>().unwrap();
                    let b2 = b2.parse::<i32>().unwrap();

                    let list = vec![a1, b1, a2, b2];
                    let max = list.iter().max().unwrap();
                    let min = list.iter().min().unwrap();

                    if max - min <= b1 - a1 + b2 - a2 {
                        cnt2 += 1;
                    }


                    if (a1 <= a2 && b1 >= b2) || (a2 <= a1 && b2 >= b1) {
                        cnt += 1;
                    }
                } else {
                    // ending line
                }
            }
            println!("total: {}", cnt);
            println!("total: {}", cnt2);
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}
