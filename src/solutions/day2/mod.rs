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

    match fs::read_to_string("./input/day2/input") {
        Ok(content) => {
            let mut sum = 0;
            for line in content.split("\n") {
                let case = line.trim();
                if let Some(result) = map.get(case) {
                    sum += result;
                }
            }
            println!("Sum: {}", sum);
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}
