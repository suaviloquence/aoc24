use std::collections::{BTreeSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::grid::{Grid, Vector2};

type Inp = Grid;

fn parse_inner(input: &str) -> Inp {
    Grid::parse(input)
}

type P1 = usize;
fn p1_inner(input: &Inp) -> P1 {
    input
        .enumerate()
        .filter(|(_, x)| **x == '0')
        .map(|(j, _)| {
            let mut st = BTreeSet::new();

            let mut q = VecDeque::new();
            q.push_back((j, b'0'));

            while let Some((j, c)) = q.pop_front() {
                if c == b'9' {
                    st.insert(j);
                }
                let next = (c + 1) as char;

                for d in Vector2::CARDINALS {
                    let v = j.wrapping_add_i(d);

                    if input.get(v) == Some(&next) {
                        q.push_back((v, next as u8));
                    }
                }
            }

            st.len()
        })
        .sum()
}

type P2 = usize;
fn p2_inner(input: &Inp) -> P2 {
    let mut scores = input.clone().map(|c| if c == '9' { 1 } else { 0 });

    for c in (b'0'..=b'8').rev() {
        let next = (c + 1) as char;
        let c = c as char;
        for (j, _) in input.enumerate().filter(|(_, x)| **x == c) {
            for d in Vector2::CARDINALS {
                let v = j.wrapping_add_i(d);

                if input.get(v) == Some(&next) {
                    scores[j] += scores[v];
                }
            }
        }
    }

    input
        .enumerate()
        .filter_map(|(j, &c)| (c == '0').then(|| scores[j]))
        .sum()
}

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Inp {
    parse_inner(input)
}

#[aoc(day10, part1)]
pub fn p1(input: &Inp) -> usize {
    p1_inner(input)
}

#[aoc(day10, part2)]
pub fn p2(input: &Inp) -> usize {
    p2_inner(input)
}

const INP: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
fn t1() {
    panic!("{}", p1(&parse(INP)));
}

#[test]
fn t2() {
    panic!("{}", p2(&parse(INP)));
}
