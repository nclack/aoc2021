use anyhow::{anyhow, Result};
pub struct Part1;
pub struct Part2;

#[derive(Debug, Default, PartialEq, Eq)]
struct Board(Vec<Vec<u8>>);

#[derive(Debug)]
struct Input {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

#[derive(Debug, Default, Clone)]
struct State {
    unmarked_sum: u16,
    column_hits: [u8; 5],
    row_hits: [u8; 5],
    done: bool,
}

pub mod parse {

    use nom::{
        bytes::complete::tag,
        character::{
            complete::{digit1, multispace0},
            streaming::line_ending,
        },
        combinator::{map, map_res, opt},
        multi::{many0, many1, many_m_n},
        sequence::{delimited, pair, terminated},
        IResult,
    };

    use super::{Board, Input};

    fn numbers(input: &str) -> IResult<&str, Vec<u8>> {
        many1(terminated(
            map_res(digit1, |e: &str| e.parse::<u8>()),
            opt(tag(",")),
        ))(input)
    }

    #[test]
    fn test_numbers() {
        assert_eq!(numbers("13,47,64,52").unwrap(), ("", vec![13, 47, 64, 52]));
    }

    fn board(input: &str) -> IResult<&str, Board> {
        let entry = delimited(
            multispace0,
            map_res(digit1, |e: &str| e.parse::<u8>()),
            multispace0,
        );
        let row = many_m_n(5, 5, entry);
        map(many_m_n(5, 5, row), Board)(input)
    }

    #[test]
    fn test_board() {
        let input = r"35 48 10 81 60
25 86 24 43 15
44 55 12 54 62
94 89 95  2 23
64 63 45 50 66";
        let out: Vec<Vec<u8>> = vec![
            vec![35, 48, 10, 81, 60],
            vec![25, 86, 24, 43, 15],
            vec![44, 55, 12, 54, 62],
            vec![94, 89, 95, 2, 23],
            vec![64, 63, 45, 50, 66],
        ];
        assert_eq!(board(&input).unwrap(), ("", Board(out)));
    }

    pub(super) fn data(input: &str) -> IResult<&str, Input> {
        map(
            pair(terminated(numbers, many0(line_ending)), many1(board)),
            |(numbers, boards)| Input { numbers, boards },
        )(input)
    }
}

impl Board {
    fn hit(&self, n: u8) -> Option<(usize, usize)> {
        for r in 0..5 {
            for c in 0..5 {
                if self.0[r][c] == n {
                    return Some((r, c));
                }
            }
        }
        None
    }
}

impl State {
    fn new(board: &Board) -> Self {
        State {
            unmarked_sum: board
                .0
                .iter()
                .map::<u16, _>(|row| row.iter().map(|e| *e as u16).sum())
                .sum(),
            ..Default::default()
        }
    }
}

impl Part2 {
    pub fn solve(input: &str) -> Result<usize> {
        let (rest, input) = parse::data(input).unwrap();
        assert!(rest.len() == 0);

        let mut remaining = input.boards.len();
        let mut states: Vec<State> = input.boards.iter().map(|b| State::new(b)).collect();
        for n in input.numbers {
            for (board, state) in input
                .boards
                .iter()
                .zip(states.iter_mut())
                .filter(|(_, s)| !s.done)
            {
                if let Some((r, c)) = board.hit(n) {
                    state.unmarked_sum -= n as u16;
                    state.row_hits[r] += 1;
                    state.column_hits[c] += 1;
                    if state.row_hits[r] == 5 || state.column_hits[c] == 5 {
                        state.done = true;
                        remaining -= 1;
                    }
                    if remaining == 0 {
                        return Ok(state.unmarked_sum as usize * n as usize);
                    }
                }
            }
        }
        Err(anyhow!("no winner"))
    }
}

impl Part1 {
    pub fn solve(input: &str) -> Result<usize> {
        let (rest, input) = parse::data(input).unwrap();
        assert!(rest.len() == 0);

        let mut states: Vec<State> = input.boards.iter().map(|b| State::new(b)).collect();
        for n in input.numbers {
            for (board, state) in input.boards.iter().zip(states.iter_mut()) {
                if let Some((r, c)) = board.hit(n) {
                    state.unmarked_sum -= n as u16;
                    state.row_hits[r] += 1;
                    state.column_hits[c] += 1;
                    if state.row_hits[r] == 5 || state.column_hits[c] == 5 {
                        return Ok(state.unmarked_sum as usize * n as usize);
                    }
                }
            }
        }
        Err(anyhow!("no winner"))
    }
}
#[test]
fn day4() {
    assert_eq!(
        4512,
        Part1::solve(include_str!("../assets/day4.0.test.txt")).unwrap()
    );
    assert_eq!(
        1924,
        Part2::solve(include_str!("../assets/day4.0.test.txt")).unwrap()
    );
}
