mod day1;
mod day2;
mod day3;

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
    );
}
