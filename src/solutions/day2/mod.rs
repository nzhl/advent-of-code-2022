use std::fs;
use std::collections::HashMap;

pub fn solve() {
    let mut map = HashMap::new();
    map.insert("A X", 1 + 3);
    map.insert("A Y", 2 + 6);
    map.insert("A Z", 3 + 0);
    map.insert("B X", 1 + 0);
    map.insert("B Y", 2 + 3);
    map.insert("B Z", 3 + 6);
    map.insert("C X", 1 + 6);
    map.insert("C Y", 2 + 0);
    map.insert("C Z", 3 + 3);


    let mut map2 = HashMap::new();
    map2.insert("A X", 3 + 0);
    map2.insert("A Y", 1 + 3);
    map2.insert("A Z", 2 + 6);
    map2.insert("B X", 1 + 0);
    map2.insert("B Y", 2 + 3);
    map2.insert("B Z", 3 + 6);
    map2.insert("C X", 2 + 0);
    map2.insert("C Y", 3 + 3);
    map2.insert("C Z", 1 + 6);

    match fs::read_to_string("./input/day2/input") {
        Ok(content) => {
            // part1 & part2
            let mut sum = 0;
            let mut sum2 = 0;
            for line in content.split("\n") {
                let case = line.trim();
                if let Some(result) = map.get(case) {
                    sum += result;
                }
                if let Some(result) = map2.get(case) {
                    sum2 += result;
                }
            }
            println!("Sum: {} {}", sum, sum2);

        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }

}
