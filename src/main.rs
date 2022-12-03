use std::env;
use solutions::solve;


mod solutions;


fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(n) = args[1].parse::<u32>() {
        solve(n);
    }  else {
        eprintln!("Try to pass question number i.e. cargo run 1")
    }
}
