use std::fs;
use std::collections::HashMap;

fn char2priority(ch: &char) -> u32 {
    if ch.is_lowercase() {
        ch.clone() as u32 - 'a' as u32 + 1
    } else {
        ch.clone() as u32 - 'A' as u32 + 27
    }
}

pub fn solve() {
    match fs::read_to_string("./input/day3/input") {
        Ok(content) => {
            let mut badge_map = HashMap::new();
            let mut index_in_group = 1;
            let mut sum2 = 0;

            let mut sum = 0;
            for line in content.split("\n") {
                let len = line.trim().len();
                let mut duplicate_map = HashMap::new();

                // part1
                for i in 0..len {
                    let ch = line.chars().nth(i).unwrap();
                    if i < len / 2 {
                        duplicate_map.insert(ch, 1);
                    } else {
                        if let Some(1) = duplicate_map.get(&ch) {
                            // aovid recalc the same char
                            duplicate_map.insert(ch, 2);
                            sum += char2priority(&ch);
                        }
                    }
                }


                // part2
                for i in 0..len {

                    let ch = line.chars().nth(i).unwrap();
                    if let Some(count) = badge_map.get(&ch) {
                        if index_in_group == 2 && *count == 1 {
                            badge_map.insert(ch, 2);
                        } else if index_in_group == 3 && *count == 2 {
                            // badge found, add & reset
                            sum2 += char2priority(&ch);
                            index_in_group = 0;
                            badge_map.clear();
                            break;
                        }

                    } else if index_in_group == 1 {
                        badge_map.insert(ch, 1);
                    }
                }

                index_in_group = index_in_group + 1;
            }
            println!("Sum: {}", sum);
            println!("Sum2: {}", sum2);
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}
