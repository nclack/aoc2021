mod day1;
use anyhow::Result;

trait Solution {
    fn solve(input: &str) -> Result<i64>;
}

fn main() {
    println!(
        "day 1.0\t{:?}",
        day1::Part1::solve(include_str!("../assets/day1.0.txt"))
    );
    println!(
        "day 1.1\t{:?}",
        day1::Part2::solve(include_str!("../assets/day1.0.txt"))
    );
}
