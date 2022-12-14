use std::fs;
use std::vec::Vec;
use std::collections::HashMap;

fn part1() {
    match fs::read_to_string("./input/day8/input") {
        Ok(content) => {
            // build the grid in i32
            let mut rows = vec![];
            let mut columns: Vec<Vec<i32>> = vec![];
            for row in content.split("\n").filter(|each|each.len() != 0).map(|line|{
                return line.split("").filter_map(|each|each.parse::<i32>().ok()).collect::<Vec<_>>();
            }) {
                rows.push(row.clone());
                for idx in 0..row.len() {
                    let column = if columns.len() > idx {
                        &mut columns[idx]
                    } else {
                        columns.push(vec![]);
                        &mut columns[idx]
                    };
                    column.push(row[idx]);
                }
            }

            let mut count = 0;
            let mut map = HashMap::new();

            fn get_map_key (i: usize, j: usize) -> String {
                format!("[{}][{}]", i, j)
            }


            for i in 0..rows.len() {
                let len = rows[i].len();
                let mut cur_max_from_left = -1;
                let mut cur_max_from_right = -1;
                for j in 0..len {
                    if rows[i][j] > cur_max_from_left {
                        let map_key = get_map_key(i, j);
                        if let None = map.get(&map_key) {
                            map.insert(map_key, true);
                            count += 1;
                        }
                        cur_max_from_left = rows[i][j];
                    }

                    if rows[i][len - j - 1] > cur_max_from_right {
                        let map_key = get_map_key(i, len - j - 1);
                        if let None = map.get(&map_key) {
                            map.insert(map_key, true);
                            count += 1;
                        }
                        cur_max_from_right = rows[i][len - j - 1];
                    }
                }
            }


            for i in 0..columns.len() {
                let len = columns[i].len();
                let mut cur_max_from_left = -1;
                let mut cur_max_from_right = -1;
                for j in 0..len {
                    if columns[i][j] > cur_max_from_left {
                        let map_key = get_map_key(j, i);
                        if let None = map.get(&map_key) {
                            map.insert(map_key, true);
                            count += 1;
                        }
                        cur_max_from_left = columns[i][j];
                    }

                    if columns[i][len - j - 1] > cur_max_from_right {
                        let map_key = get_map_key(len - j - 1, i);
                        if let None = map.get(&map_key) {
                            map.insert(map_key, true);
                            count += 1;
                        }
                        cur_max_from_right = columns[i][len - j - 1];
                    }
                }
            }

            println!("count {}", count);

        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }

}

pub fn solve() {
    part1();
}
