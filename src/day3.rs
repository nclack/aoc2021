use std::{cell::RefCell, rc::Rc};

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
pub struct Part2;

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

#[derive(Default, Debug)]
struct Node {
    count: usize,
    children: (Option<Rc<RefCell<Node>>>, Option<Rc<RefCell<Node>>>),
}

trait TreeAccumulator {
    fn one(&self) -> Rc<RefCell<Node>>;
    fn zero(&self) -> Rc<RefCell<Node>>;
}

impl Node {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Default::default()))
    }
}
impl TreeAccumulator for Rc<RefCell<Node>> {
    fn one(&self) -> Rc<RefCell<Node>> {
        let mut acc = self.as_ref().borrow_mut();
        acc.count += 1;
        match &acc.children.1 {
            Some(child) => child.clone(),
            None => {
                let child = Node::new();
                acc.children.1 = Some(child.clone());
                child
            }
        }
    }

    fn zero(&self) -> Rc<RefCell<Node>> {
        let mut acc = self.as_ref().borrow_mut();
        acc.count += 1;
        match &acc.children.0 {
            Some(child) => child.clone(),
            None => {
                let child = Node::new();
                acc.children.0 = Some(child.clone());
                child
            }
        }
    }
}

impl Part2 {
    pub fn solve(input: &str) -> Result<usize> {
        // Want to build a binary tree and keep track of how many children are
        // down each branch.
        fn parse(input: &str) -> IResult<&str, (usize, usize)> {
            let bits = many1(alt((value(0i8, tag("0")), value(1, tag("1")))));
            map(
                fold_many1(terminated(bits, opt(line_ending)), Node::new, |acc, n| {
                    n.into_iter().fold(
                        acc.clone(),
                        |acc, b| {
                            if b == 0 {
                                acc.zero()
                            } else {
                                acc.one()
                            }
                        },
                    );
                    acc
                }),
                |tree| {
                    let ox = readout(&tree, |n0, n1| n1 >= n0);
                    let co2 = readout(&tree, |n0, n1| n1 < n0);
                    (ox, co2)
                },
            )(input)
        }

        let (rest, (ox, co2)) = parse(input).unwrap();
        assert!(rest.len() == 0);
        Ok(ox * co2)
    }
}

/// Readout bits from the tree picking branches with 'pred'.
fn readout<F>(tree: &Rc<RefCell<Node>>, pred: F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    fn count(n: &Option<Rc<RefCell<Node>>>) -> Option<usize> {
        n.as_ref().map(|n| n.borrow().count)
    }
    fn child_counts(n: Rc<RefCell<Node>>) -> (Option<usize>, Option<usize>) {
        let (l, r) = &n.borrow().children;
        (count(&l), count(&r))
    }

    let mut out = 0;
    let mut cur = Some(tree.clone());
    while let Some(node) = cur.as_ref() {
        let (next, bit) = match child_counts(node.clone()) {
            (Some(n0), Some(n1)) if pred(n0, n1) => (node.borrow().children.1.clone(), 1),
            (None, Some(_)) => (node.borrow().children.1.clone(), 1),
            (Some(_), _) => (node.borrow().children.0.clone(), 0),
            _ => (None, 0),
        };
        cur = next;
        if cur.is_some() {
            out = (out << 1) + bit;
        }
    }
    out
}

#[test]
fn day3() {
    assert_eq!(
        198,
        Part1::solve(include_str!("../assets/day3.0.test.txt")).unwrap()
    );
    assert_eq!(
        230,
        Part2::solve(include_str!("../assets/day3.0.test.txt")).unwrap()
    );
}

#[test]
fn test_tree() {
    let acc = Node::new();
    acc.one().one().zero();
    assert_eq!(1, acc.borrow().count);
    assert_eq!(1, acc.borrow().children.1.as_ref().unwrap().borrow().count);
    assert_eq!(
        1,
        acc.borrow()
            .children
            .1
            .as_ref()
            .unwrap()
            .borrow()
            .children
            .1
            .as_ref()
            .unwrap()
            .borrow()
            .count
    );
    assert_eq!(
        0,
        acc.borrow()
            .children
            .1
            .as_ref()
            .unwrap()
            .borrow()
            .children
            .1
            .as_ref()
            .unwrap()
            .borrow()
            .children
            .0
            .as_ref()
            .unwrap()
            .borrow()
            .count
    );
    acc.one().zero().zero();
    assert_eq!(2, acc.borrow().count);
    assert_eq!(2, acc.borrow().children.1.as_ref().unwrap().borrow().count);
}
