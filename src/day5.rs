use std::collections::HashMap;

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
pub struct Part1;
pub struct Part2;

#[derive(Debug)]
struct Point((i32, i32));
#[derive(Debug)]
struct Line((Point, Point));

#[derive(Debug)]
struct LinePoints {
    start: (i32, i32),
    dr: (i32, i32),
    i: i32,
    n: i32,
}

impl Iterator for LinePoints {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i <= self.n {
            let pt = (
                self.start.0 + self.dr.0 * self.i,
                self.start.1 + self.dr.1 * self.i,
            );
            self.i += 1;
            Some(pt)
        } else {
            None
        }
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        let (Point((r0x, r0y)), Point((r1x, r1y))) = self.0;
        r0x == r1x || r0y == r1y
    }

    fn iter(&self) -> LinePoints {
        let (Point((r0x, r0y)), Point((r1x, r1y))) = self.0;
        let (dx, dy) = (r1x - r0x, r1y - r0y);

        // we know lines are either horizontal, vertical or diagonal
        LinePoints {
            start: (r0x, r0y),
            dr: (dx.signum(), dy.signum()),
            i: 0,
            n: dx.abs().max(dy.abs()),
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    fn number(input: &str) -> IResult<&str, i32> {
        map_res(digit1, |e: &str| e.parse::<i32>())(input)
    }
    fn point(input: &str) -> IResult<&str, Point> {
        map(separated_pair(number, tag(","), number), Point)(input)
    }
    let line = map(separated_pair(point, tag(" -> "), point), Line);
    many1(terminated(line, opt(line_ending)))(input)
}

impl Part1 {
    pub fn solve(input: &str) -> Result<usize> {
        let (rest, lines) = parse(input).unwrap();
        assert!(rest.len() == 0);

        let mut hist = HashMap::new();
        for line in lines.iter().filter(|&l| l.is_straight()) {
            for p in line.iter() {
                *hist.entry(p).or_insert(0) += 1;
            }
        }

        Ok(hist.iter().filter(|(_, &v)| v > 1).count())
    }
}

impl Part2 {
    pub fn solve(input: &str) -> Result<usize> {
        let (rest, lines) = parse(input).unwrap();
        assert!(rest.len() == 0);

        let mut hist = HashMap::new();
        for line in lines.iter() {
            for p in line.iter() {
                *hist.entry(p).or_insert(0) += 1;
            }
        }

        Ok(hist.iter().filter(|(_, &v)| v > 1).count())
    }
}

#[test]
fn day5() {
    assert_eq!(
        5,
        Part1::solve(include_str!("../assets/day5.0.test.txt")).unwrap()
    );
    assert_eq!(
        12,
        Part2::solve(include_str!("../assets/day5.0.test.txt")).unwrap()
    );
}
