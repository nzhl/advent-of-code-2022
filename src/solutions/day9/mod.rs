use std::fs;
use std::collections::HashMap;

pub fn get_distance (a: (i32, i32), b: (i32, i32)) -> f64 {
    let distance = (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f64).sqrt();
    distance
}

pub fn part1() {
    match fs::read_to_string("./input/day9/input") {
        Ok(content) => {
            let mut head_pos = (0, 0);
            let mut tail_pos = (0, 0);
            let mut count = 1;
            let mut map = HashMap::new();
            map.insert(tail_pos, true);
            for line in content.split("\n") {
                if line.trim().len() != 0 {
                    let direction = &line[0..1];
                    let steps = line[2..].parse::<usize>().unwrap();

                    for _ in 0..steps {
                        head_pos = match direction {
                            "U" => (head_pos.0, head_pos.1 + 1),
                            "D" => (head_pos.0, head_pos.1 - 1),
                            "L" => (head_pos.0 - 1, head_pos.1),
                            "R" => (head_pos.0 + 1, head_pos.1),
                            dir => panic!("Unexpected direction:  {}!", dir)
                        };

                        if get_distance(head_pos, tail_pos) < 2.0 {
                            // adjacent
                            continue;
                        }

                        if head_pos.0 == tail_pos.0 {
                            tail_pos.1 = (head_pos.1 + tail_pos.1) / 2;
                        } else if head_pos.1 == tail_pos.1 {
                            tail_pos.0 = (head_pos.0 + tail_pos.0) / 2;
                        } else {
                            // diagonally 
                            tail_pos.0 = if head_pos.0 > tail_pos.0 { tail_pos.0 + 1 } else { tail_pos.0 - 1 };
                            tail_pos.1 = if head_pos.1 > tail_pos.1 { tail_pos.1 + 1 } else { tail_pos.1 - 1 };
                        }
                        if let None = map.get(&tail_pos) {
                            count += 1;
                            map.insert(tail_pos, true);
                        }
                    }
                }
            }
            println!("count: {}", count);
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }
}

const NUM_OF_KNOTS: u32 = 10;

pub fn part2() {
    match fs::read_to_string("./input/day9/input") {
        Ok(content) => {
            let mut rope = {
                let mut knots = Vec::new();
                for _ in 0..NUM_OF_KNOTS {
                    knots.push((0, 0));
                }
                knots
            };
            let mut count = 1;
            let mut map = HashMap::new();
            map.insert(rope.last().unwrap().clone(), true);
            for line in content.split("\n") {
                if line.trim().len() != 0 {
                    let direction = &line[0..1];
                    let steps = line[2..].parse::<usize>().unwrap();

                    for _ in 0..steps {
                        let mut head_pos = (0, 0);
                        for (idx, knot) in rope.iter_mut().enumerate() {
                            if idx == 0 {
                                // head
                                *knot = match direction {
                                    "U" => (knot.0, knot.1 + 1),
                                    "D" => (knot.0, knot.1 - 1),
                                    "L" => (knot.0 - 1, knot.1),
                                    "R" => (knot.0 + 1, knot.1),
                                    dir => panic!("Unexpected direction:  {}!", dir)
                                };
                            } else {
                                let tail_pos = *knot;
                                if get_distance(head_pos, tail_pos) >= 2.0 {
                                    // not adjacent
                                    if head_pos.0 == tail_pos.0 {
                                        knot.1 = (head_pos.1 + tail_pos.1) / 2;
                                    } else if head_pos.1 == tail_pos.1 {
                                        knot.0 = (head_pos.0 + tail_pos.0) / 2;
                                    } else {
                                        // diagonally 
                                        knot.0 = if head_pos.0 > tail_pos.0 { tail_pos.0 + 1 } else { tail_pos.0 - 1 };
                                        knot.1 = if head_pos.1 > tail_pos.1 { tail_pos.1 + 1 } else { tail_pos.1 - 1 };
                                    }
                                }
                            }
                            head_pos = *knot;
                        }
                        // tail
                        if let None = map.get(rope.last().unwrap()) {
                            count += 1;
                            map.insert(rope.last().unwrap().clone(), true);
                        }
                    }
                } else {
                    // skip
                }
            }
            println!("count: {}", count);
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
