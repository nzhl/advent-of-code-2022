use std::fs;
use std::vec::Vec;
use std::collections::HashMap;


type Grid = Vec<Vec<i32>>;

fn build_grid(content: &String) -> (Grid, Grid) {
    let mut rows = vec![];
    let mut columns: Grid  = vec![];
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
    return (rows, columns);
}


fn get_distance (map: &HashMap<String, usize>, i: usize, j: usize) -> Option<&usize> {
    let key = format!("[{}][{}]", i, j);
    return map.get(&key);
}

fn set_distance (map: &mut HashMap<String, usize>, i: usize, j: usize, value: usize) {
    let key = format!("[{}][{}]", i, j);
    map.insert(key, value);
}



fn part1() {
    match fs::read_to_string("./input/day8/input") {
        Ok(content) => {
            // build the grid in i32
            let (rows, columns) = build_grid(&content);

            let mut count = 0;
            let mut map = HashMap::new();
            for i in 0..rows.len() {
                let len = rows[i].len();
                let mut cur_max_from_left = -1;
                let mut cur_max_from_right = -1;
                for j in 0..len {
                    if rows[i][j] > cur_max_from_left {
                        if let None = get_distance(&map, i, j) {
                            set_distance(&mut map, i, j, 1);
                            count += 1;
                        }
                        cur_max_from_left = rows[i][j];
                    }

                    if rows[i][len - j - 1] > cur_max_from_right {
                        if let None = get_distance(&map, i, len - j - 1) {
                            set_distance(&mut map, i, len - j - 1, 1);
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
                        if let None = get_distance(&map, j, i) {
                            set_distance(&mut map, j, i, 1);
                            count += 1;
                        }
                        cur_max_from_left = columns[i][j];
                    }

                    if columns[i][len - j - 1] > cur_max_from_right {
                        if let None = get_distance(&map, len - j - 1, i) {
                            set_distance(&mut map, len - j - 1, i, 1);
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


fn build_distance_map (grid: &Grid, reverse_horizontal: bool, is_column_grid: bool) -> HashMap<String, usize>  {
    let mut distance_map = HashMap::new();

    for i in 0..grid.len() {
        let mut ascending_stack = vec![];
        let row: Vec<&i32> = if reverse_horizontal {
            grid[i].iter().rev().collect()
        } else {
            grid[i].iter().collect()
        };
        for j in 0..row.len() {
            let cur_tree_height = *row[j];
            while ascending_stack.len() != 0 && *(row[*ascending_stack.last().unwrap()] as &i32) <= cur_tree_height {
                let idx = ascending_stack.pop().unwrap();
                if reverse_horizontal && !is_column_grid {
                    set_distance(&mut distance_map, i, row.len() - 1 - idx, j - idx)
                } else if !reverse_horizontal && !is_column_grid {
                    set_distance(&mut distance_map, i, idx, j - idx)
                } else if reverse_horizontal && is_column_grid {
                    set_distance(&mut distance_map, row.len() - 1 - idx, i, j - idx)
                } else {
                    set_distance(&mut distance_map, idx, i, j - idx)
                }
            }
            ascending_stack.push(j);
        }
        while ascending_stack.len() != 0 {
            let idx = ascending_stack.pop().unwrap();
            if reverse_horizontal && !is_column_grid {
                set_distance(&mut distance_map, i, row.len() - 1 - idx, row.len() - 1 - idx);
            } else if !reverse_horizontal && !is_column_grid {
                set_distance(&mut distance_map, i, idx, row.len() - 1 - idx);
            } else if reverse_horizontal && is_column_grid {
                set_distance(&mut distance_map, row.len() - 1 - idx, i, row.len() - 1 - idx);
            } else {
                set_distance(&mut distance_map, idx, i, row.len() - 1 - idx);
            }
        }
    }

    distance_map
}

fn part2() {
    match fs::read_to_string("./input/day8/input") {
        Ok(content) => {
            // build the grid in i32
            let (rows, columns) = build_grid(&content);
            let vec_len = rows.len();
            let hoz_len = columns.len();

            let distance_to_right_larger_map = build_distance_map(&rows, false, false);
            let distance_to_left_larger_map = build_distance_map(&rows, true, false);
            let distance_to_down_larger_map = build_distance_map(&columns, false, true);
            let distance_to_up_larger_map = build_distance_map(&columns, true, true);

            let mut highest = 0;
            for i in 0..vec_len {
                for j in 0..hoz_len {
                    let left = get_distance(&distance_to_left_larger_map, i, j).unwrap();
                    let right = get_distance(&distance_to_right_larger_map, i, j).unwrap();
                    let up = get_distance(&distance_to_up_larger_map, i, j).unwrap();
                    let down = get_distance(&distance_to_down_larger_map, i, j).unwrap();

                    if left * right * up * down > highest {
                        highest = left * right * up * down
                    }
                }
            }

            // println!("aaaa {:?}", distance_to_down_larger_map.get(&get_map_key(0, 0)));
            // println!("aaaa {:?}", distance_to_down_larger_map.get(&get_map_key(0, 1)));
            // println!("aaaa {:?}", distance_to_down_larger_map.get(&get_map_key(0, 2)));
            // println!("aaaa {:?}", distance_to_down_larger_map.get(&get_map_key(0, 3)));
            // println!("aaaa {:?}", distance_to_down_larger_map.get(&get_map_key(0, 4)));
            // println!("aaaa {:?}", distance_to_down_larger_map.get(&get_map_key(0, 5)));
            // println!("aaaa {:?}", distance_to_down_larger_map.get(&get_map_key(0, 6)));

            println!("highest: {}", highest);

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
