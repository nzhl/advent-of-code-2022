pub mod day1;



pub fn solve(num: u32) {
    match num {
        1 => day1::solve(),
        
        _ => {
            eprintln!("Unsolved problem {} !", num);
        }
    };
}
