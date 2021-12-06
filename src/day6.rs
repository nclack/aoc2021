use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    multi::many1,
    sequence::terminated,
    IResult,
};

pub struct Part1;

pub struct Part2;

fn parse(input: &str) -> IResult<&str, Vec<u8>> {
    many1(terminated(
        map_res(digit1, |e: &str| e.parse::<u8>()),
        opt(tag(",")),
    ))(input)
}

impl Part1 {
    pub fn solve(input: &str) -> Result<usize> {
        let (_rest, fish) = parse(input).unwrap();

        Ok(simulate(fish, 80))
    }
}

impl Part2 {
    pub fn solve(input: &str) -> Result<usize> {
        let (_rest, fish) = parse(input).unwrap();

        Ok(simulate(fish, 256))
    }
}

fn simulate(fish: Vec<u8>, days: usize) -> usize {
    let mut counts: Vec<_> = vec![0; 9];
    for f in fish {
        counts[f as usize] += 1;
    }
    for _day in 0..days {
        let dividing = counts[0];
        counts.rotate_left(1);
        counts[6] += dividing;
    }
    counts.iter().sum()
}

#[test]
fn day6() {
    assert_eq!(
        5934,
        Part1::solve(include_str!("../assets/day6.0.test.txt")).unwrap()
    );
    assert_eq!(
        26984457539,
        Part2::solve(include_str!("../assets/day6.0.test.txt")).unwrap()
    );
}
