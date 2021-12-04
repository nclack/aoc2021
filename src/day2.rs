use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    combinator::{map, map_res, opt, value},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub struct Part1;
pub struct Part2;

#[derive(Clone, Copy, Debug)]
enum Dir {
    F,
    U,
    D,
}

#[derive(Debug)]
struct Move {
    dir: Dir,
    amt: i64,
}

#[derive(Debug, Default)]
struct State {
    depth: i64,
    x: i64,
}

impl State {
    fn update(self, m: &Move) -> State {
        let State { depth, x } = self;
        let (x, depth) = match m {
            Move { dir: Dir::U, amt } => (x, depth - amt),
            Move { dir: Dir::D, amt } => (x, depth + amt),
            Move { dir: Dir::F, amt } => (self.x + amt, depth),
        };
        State { depth, x }
    }
}

#[derive(Debug, Default)]
struct State2 {
    state: State,
    aim: i64,
}

impl State2 {
    fn update(self, m: &Move) -> State2 {
        let State2 {
            state: State { depth, x },
            aim,
        } = self;
        let (x, depth, aim) = match m {
            Move { dir: Dir::U, amt } => (x, depth, aim - amt),
            Move { dir: Dir::D, amt } => (x, depth, aim + amt),
            Move { dir: Dir::F, amt } => (x + amt, depth + aim * amt, aim),
        };
        let state = State { depth, x };
        State2 { state, aim }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Move>> {
    let mv = alt((
        value(Dir::F, tag("forward")),
        value(Dir::U, tag("up")),
        value(Dir::D, tag("down")),
    ));
    let number = map_res(digit1, |e| i64::from_str_radix(e, 10));

    many1(terminated(
        map(separated_pair(mv, multispace1, number), |(dir, amt)| Move {
            dir,
            amt,
        }),
        opt(line_ending),
    ))(input)
}

impl Part1 {
    pub fn solve(input: &str) -> Result<i64> {
        let (rest, moves) = parse(input).unwrap();
        assert!(rest.len() == 0);

        let state = moves.into_iter().fold(
            State {
                ..Default::default()
            },
            |acc, el| acc.update(&el),
        );

        Ok(state.x * state.depth)
    }
}

impl Part2 {
    pub fn solve(input: &str) -> Result<i64> {
        let (rest, moves) = parse(input).unwrap();
        assert!(rest.len() == 0);

        let state = moves.into_iter().fold(
            State2 {
                ..Default::default()
            },
            |acc, el| acc.update(&el),
        );

        let state = state.state;
        Ok(state.x * state.depth)
    }
}

#[test]
fn test_parse() {
    let (rest, _) = parse(include_str!("../assets/day2.0.txt")).unwrap();
    assert!(rest.len() == 0);
}

#[test]
fn test_solve() {
    assert_eq!(
        150,
        Part1::solve(include_str!("../assets/day2.0.test.txt")).unwrap()
    );
    assert_eq!(
        900,
        Part2::solve(include_str!("../assets/day2.0.test.txt")).unwrap()
    );
}
