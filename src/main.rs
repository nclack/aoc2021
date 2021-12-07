mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

macro_rules! problems {
    ()=>{};
    ($day:tt $($parts:ident)+, $($rest:tt)*) => {
        problems!($day $($parts)+);
        problems!($($rest)*);
    };
    ($day:tt $part:tt $($rest:tt)+) => {
        problems!($day $part);
        problems!($day $($rest)+);
    };
    ($day:tt $part:tt)=>{
        println!(
            "{} {}\t{:?}",
            stringify!($day),
            stringify!($part),
            $day::$part::solve(include_str!(concat!("../assets/",stringify!($day),".0.txt")))
        );
    };
}

fn main() {
    problems!(
        day1 Part1 Part2,
        day2 Part1 Part2,
        day3 Part1 Part2,
        day4 Part1 Part2,
        day5 Part1 Part2,
        day6 Part1 Part2,
        day7 Part1 Part2
    );
}
