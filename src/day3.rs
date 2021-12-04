use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{map, opt, value},
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult,
};

pub struct Part1;

impl Part1 {
    pub fn solve(input: &str) -> Result<u64> {
        /// returns (mask, gamma)
        fn parse(input: &str) -> IResult<&str, (u64, u64)> {
            let bits = many1(alt((value(-1i8, tag("0")), value(1, tag("1")))));
            map(
                fold_many1(
                    terminated(bits, opt(line_ending)),
                    Vec::new,
                    |mut acc: Vec<i8>, n| {
                        acc.resize(n.len(), 0);
                        for (a, b) in acc.iter_mut().zip(n.iter()) {
                            *a = *a + b;
                        }
                        acc
                    },
                ),
                |bits| {
                    (
                        (1 << bits.len()) - 1,
                        bits.into_iter()
                            .fold(0, |acc, b| (acc << 1) + ((b > 0) as u64)),
                    )
                },
            )(input)
        }

        let (rest, (mask, gamma)) = parse(input).unwrap();
        assert!(rest.len() == 0);
        let epsilon = !gamma & mask;
        Ok(gamma * epsilon)
    }
}

#[test]
fn day3() {
    assert_eq!(
        198,
        Part1::solve(include_str!("../assets/day3.0.test.txt")).unwrap()
    );
}
