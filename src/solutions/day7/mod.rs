use std::cell::RefCell;
use std::{fs, usize};
use std::vec::Vec;
use std::rc::Rc;
use std::iter;


#[derive(Debug)]
pub enum FsNode {
    File {
        name: String,
        size: usize,
    },
    Dir {
        name: String,
        size: usize,
        children: Vec<Rc<RefCell<FsNode>>>
    },
}

impl FsNode {
    fn new(is_dir: bool, name: String, size: usize) -> FsNode {
        if is_dir {
            FsNode::Dir { name, size, children: vec![] }
        } else {
            FsNode::File { name, size }
        }
    }

    fn append_child(self: &mut Self, node: FsNode) {
        if let FsNode::Dir{children, ..} = self {
            children.push(Rc::new(RefCell::new(node)));
        }
    }

    fn get_name(self: &Self) -> String {
        match self {
            FsNode::Dir { name, .. } => name.to_string(),
            FsNode::File { name, .. } => name.to_string()
        }
    }

    fn set_name(self: &mut Self, new_name: String) {
        if let FsNode::Dir{name, ..} = self {
            *name = new_name;
        }
    }


    fn find_child(self: &Self, name: String) -> Rc<RefCell<FsNode>> {
        if let FsNode::Dir{children, ..} = self {
            let idx = children.iter().position(|each|RefCell::borrow(each).get_name() == name).unwrap();
            return children.get(idx).unwrap().clone();
        }
        panic!("Can't find child named {}", name);
    }

    fn calc_size(self: &mut Self, threshould: usize, sum: &mut usize) -> usize {
        match self {
            FsNode::File { size, .. } => {
                size.clone()
            }
            FsNode::Dir { size, children, .. } => {
                *size = children.iter().fold(0, |acc, item|acc + item.borrow_mut().calc_size(threshould, sum));

                let file_size = size.clone();
                if file_size <= threshould {
                    *sum += file_size;
                }

                file_size
            }
        }
    }

    fn print(self: &Self, depth: usize) {
        let tabs = iter::repeat("  ").take(depth).collect::<String>();
        match self {
            FsNode::File { name, size } => {
                println!("{}- {} (file, size={})", tabs, name, size);
            }
            FsNode::Dir { name, size, children } => {
                println!("{}- {} (dir, size={})", tabs, name, size);
                for each in children {
                    RefCell::borrow(each).print(depth + 1);
                }
            }
        }
    }
}

pub fn part1() {
    // build the tree
    let root = Rc::new(RefCell::new(FsNode::new(true, String::new(), 0)));
    let mut cwd_traces = vec![root.clone()];
    match fs::read_to_string("./input/day7/input") {
        Ok(content) => {
            for line in content.split("\n").map(|line| line.trim()) {
                let cwd = cwd_traces.last().unwrap();
                if line.starts_with("$ cd") {
                    let name = line[5..].to_string();
                    if name == "/" {
                        cwd.borrow_mut().set_name(name);
                    } else if name == ".." {
                        cwd_traces.pop();
                    } else {
                        let target_node = RefCell::borrow(cwd).find_child(name);
                        cwd_traces.push(target_node);
                    }

                } else if line.starts_with("$ ls") {
                    // skip
                    
                } else if let Some((dir_or_size, name)) = line.split_once(" ") {
                    if dir_or_size == "dir" {
                        cwd.borrow_mut().append_child(FsNode::new(true, name.to_string(), 0))
                    } else if let Ok(size) = dir_or_size.parse::<usize>() {
                        cwd.borrow_mut().append_child(FsNode::new(false, name.to_string(), size));
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Read input error:  {}!", err);
        }
    }

    let mut sum = 0;
    // calc size
    root.borrow_mut().calc_size(100000, &mut sum);

    // print
    root.borrow().print(0);
    println!("sum {}", sum);
}

pub fn solve() {
    part1();
}
