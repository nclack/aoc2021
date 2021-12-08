use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, one_of, space0, space1},
    combinator::opt,
    multi::{count, fold_many1, many1, many1_count},
    sequence::{delimited, preceded, terminated},
    IResult,
};

pub struct Part1;

impl Part1 {
    pub fn solve(input: &str) -> Result<usize> {
        fn parse(input: &str) -> IResult<&str, usize> {
            fn series(input: &str) -> IResult<&str, usize> {
                let part = many1_count(one_of("abcdefg"));
                fold_many1(
                    terminated(part, space0),
                    || 0,
                    |acc, n| {
                        acc + match n {
                            2 | 3 | 4 | 7 => 1,
                            _ => 0,
                        }
                    },
                )(input)
            }
            let prefix = terminated(
                count(terminated(many1(one_of("abcdefg")), space0), 10),
                tag("| "),
            );
            fold_many1(
                terminated(preceded(prefix, series), opt(line_ending)),
                || 0,
                |acc, c| acc + c,
            )(input)
        }
        let (_rest, ans) = parse(input).unwrap();
        Ok(ans)
    }
}

#[test]
fn day8() {
    assert_eq!(
        26,
        Part1::solve(include_str!("../assets/day8.0.test.txt")).unwrap()
    );
    assert_eq!(
        26,
        Part2::solve(include_str!("../assets/day8.0.test.txt")).unwrap()
    );
}
