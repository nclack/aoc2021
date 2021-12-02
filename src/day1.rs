use crate::{day1, Result, Solution};
use nom::{
    character::complete::{digit1, line_ending},
    combinator::{map_res, opt},
    multi::many1,
    sequence::terminated,
    IResult,
};

pub struct Part1;
pub struct Part2;

fn parse(input: &str) -> IResult<&str, Vec<i64>> {
    many1(terminated(
        map_res(digit1, |e| i64::from_str_radix(e, 10)),
        opt(line_ending),
    ))(input)
}

impl Solution for Part1 {
    /// Input is a list of measurements.
    /// How many measurements are larger than the previous measurement?
    fn solve(input: &str) -> Result<i64> {
        let (rest, xs) = parse(input).unwrap();
        assert!(rest.len() == 0);

        let out = xs.windows(2).map(|w| (w[1] > w[0]) as i64).sum();

        Ok(out)
    }
}

impl Solution for Part2 {
    /// Input is a list of measurements.
    /// How many window measurements are larger than the previous measurement?
    fn solve(input: &str) -> Result<i64> {
        let (rest, xs) = parse(input).unwrap();
        assert!(rest.len() == 0);

        let out = xs
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<i64>>()
            .windows(2)
            .map(|w| (w[1] > w[0]) as i64)
            .sum();

        Ok(out)
    }
}

#[test]
fn day1() {
    assert_eq!(
        7,
        day1::Part1::solve(include_str!("../assets/day1.0.test.txt")).unwrap()
    );
    assert_eq!(
        5,
        day1::Part2::solve(include_str!("../assets/day1.1.test.txt")).unwrap()
    );
}
