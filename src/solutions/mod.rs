pub mod day1;
pub mod day2;
/* placeholder1 */



pub fn solve(num: u32) {
    match num {
        1 => day1::solve(),
        2 => day2::solve(),
        /* placeholder2 */
        
        _ => {
            eprintln!("Unsolved problem {} !", num);
        }
    };
}
