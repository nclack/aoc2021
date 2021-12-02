mod day1;
use anyhow::Result;

macro_rules! problems {
    ($day:tt $part:tt $input:tt, $($rest:tt)*) => {
        problems!($day $part $input);
        problems!($($rest)*);
    };
    ($day:tt $part:tt $input:tt)=>{
        println!(
            "{} {}\t{:?}",
            stringify!($day),
            stringify!($part),
            $day::$part::solve(include_str!($input))
        );
    }
}

fn main() {
    problems!(
        day1 Part1 "../assets/day1.0.txt",
        day1 Part2 "../assets/day1.0.txt"
    );
}
