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

fn parse(input: &str) -> IResult<&str, Vec<u16>> {
    many1(terminated(
        map_res(digit1, |e: &str| e.parse::<u16>()),
        opt(tag(",")),
    ))(input)
}

impl Part1 {
    pub fn solve(input: &str) -> Result<u32> {
        let (_rest, crabs) = parse(input).unwrap();
        let w = *crabs.iter().max().unwrap() as usize + 1;
        let counts = crabs.iter().fold(vec![0u32; w], |mut acc, &c| {
            acc[c as usize] += 1;
            acc
        });

        let mut left_scores = vec![0u32; w];
        let mut left = counts.clone();
        for i in 1..left.len() {
            left[i] += left[i - 1];
        }
        for i in 1..left_scores.len() {
            left_scores[i] = left_scores[i - 1] + left[i - 1];
        }

        let mut right_scores = vec![0u32; w];
        let mut right = counts.clone();
        for i in (0..right.len() - 1).rev() {
            right[i] += right[i + 1];
        }
        for i in (0..right_scores.len() - 1).rev() {
            right_scores[i] += right_scores[i + 1] + right[i + 1];
        }

        Ok(left_scores
            .into_iter()
            .zip(right_scores.into_iter())
            .map(|(l, r)| l + r)
            .min()
            .unwrap())
    }
}

impl Part2 {
    pub fn solve(input: &str) -> Result<u32> {
        let (_rest, crabs) = parse(input).unwrap();
        let w = *crabs.iter().max().unwrap() as usize + 1;
        let counts = crabs.iter().fold(vec![0u32; w], |mut acc, &c| {
            acc[c as usize] += 1;
            acc
        });

        // 1 0 0 0
        // 1 1 1 1
        //      \
        // 0 1 2-3 4
        //       |
        // 0 1 3-6

        let mut left_scores = vec![0u32; w];
        let mut left = counts.clone();
        for i in 1..left.len() {
            left[i] += left[i - 1];
        }
        for i in 1..left_scores.len() {
            left_scores[i] = left_scores[i - 1] + left[i - 1];
        }
        for i in 1..left_scores.len() {
            left_scores[i] += left_scores[i - 1];
        }

        let mut right_scores = vec![0u32; w];
        let mut right = counts.clone();
        for i in (0..right.len() - 1).rev() {
            right[i] += right[i + 1];
        }
        for i in (0..right_scores.len() - 1).rev() {
            right_scores[i] += right_scores[i + 1] + right[i + 1];
        }
        for i in (0..right_scores.len() - 1).rev() {
            right_scores[i] += right_scores[i + 1];
        }

        Ok(left_scores
            .into_iter()
            .zip(right_scores.into_iter())
            .map(|(l, r)| l + r)
            .min()
            .unwrap())
    }
}

#[test]
fn day7() {
    assert_eq!(
        37,
        Part1::solve(include_str!("../assets/day7.0.test.txt")).unwrap()
    );
    assert_eq!(
        168,
        Part2::solve(include_str!("../assets/day7.0.test.txt")).unwrap()
    );
}
