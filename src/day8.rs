use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space0,char, multispace0},
    combinator::{opt, value},
    multi::{count, fold_many1, many1, many1_count},
    sequence::{preceded, terminated, delimited, separated_pair},
    IResult, branch::alt, error::ParseError,
};

pub struct Part1;
pub struct Part2;

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

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

impl Part2 {
    fn parse(input: &str) -> IResult<&str, Vec<(Vec<u8>,Vec<u8>)>> {
        fn part(input: &str)->IResult<&str,u8> {
            fold_many1(
                alt((
                    value(1,char('a')),
                    value(2,char('b')),
                    value(4,char('c')),
                    value(8,char('d')),
                    value(16,char('e')),
                    value(32,char('f')),
                    value(64,char('g')),
                )),
                || 0,
                |acc,n| acc | n
            )(input)
        }
        let uniques=count(ws(part),10);
        let outputs=count(ws(part),4);
        let line=terminated(separated_pair(uniques,tag("|"),outputs),opt(line_ending));
        many1(line)(input)
    }

    pub fn solve(input: &str)->Result<usize> {
        let (rest,data)=Part2::parse(input).unwrap();
        assert_eq!(rest.len(),0);
        todo!()
    }
}

#[test]
fn day8() {
    assert_eq!(
        26,
        Part1::solve(include_str!("../assets/day8.0.test.txt")).unwrap()
    );
    assert_eq!(
        61229,
        Part2::solve(include_str!("../assets/day8.0.test.txt")).unwrap()
    );
}
