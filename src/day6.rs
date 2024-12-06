use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

use crate::utils::grid::{Grid, Vector2};

type Inp = Grid;

fn parse_inner(input: &str) -> Inp {
    Grid::parse(input)
}

type P1 = usize;
fn p1_inner(input: &Inp) -> P1 {
    let (st, _) = input.enumerate().find(|(_, x)| **x == '^').unwrap();

    let mut v = st;

    let mut dir = Vector2::UP;

    let mut pos = BTreeSet::new();

    loop {
        pos.insert(v);
        let next = v.wrapping_add_i(dir);

        let Some(item) = input.get(next) else {
            break;
        };

        (v, dir) = match (item, dir) {
            ('.' | '^', dir) => (next, dir),
            (_, Vector2::UP) => (v, Vector2::RIGHT),
            (_, Vector2::RIGHT) => (v, Vector2::DOWN),
            (_, Vector2::DOWN) => (v, Vector2::LEFT),
            (_, Vector2::LEFT) => (v, Vector2::UP),
            _ => unreachable!(),
        };
    }

    pos.len()
}

type P2 = usize;
fn p2_inner(input: &Inp) -> P2 {
    let (st, _) = input.enumerate().find(|(_, x)| **x == '^').unwrap();

    let cycles = |obs: Vector2| {
        let mut v = st;
        let mut dir = Vector2::UP;
        let mut cycle = false;
        let mut pos = BTreeSet::new();

        loop {
            if !pos.insert((v, dir)) {
                cycle = true;
                break;
            }

            let next = v.wrapping_add_i(dir);

            let Some(&item) = input.get(next) else {
                break;
            };

            let item = if next == obs { 'O' } else { item };

            (v, dir) = match (item, dir) {
                ('.' | '^', dir) => (next, dir),
                (_, Vector2::UP) => (v, Vector2::RIGHT),
                (_, Vector2::RIGHT) => (v, Vector2::DOWN),
                (_, Vector2::DOWN) => (v, Vector2::LEFT),
                (_, Vector2::LEFT) => (v, Vector2::UP),
                _ => unreachable!(),
            };
        }
        cycle
    };

    let dom = input.domain().filter(|x| x != &st).collect::<Vec<_>>();

    dom.into_par_iter().filter(|&v| cycles(v)).count()
}

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Inp {
    parse_inner(input)
}

#[aoc(day6, part1)]
pub fn p1(input: &Inp) -> usize {
    p1_inner(input)
}

#[aoc(day6, part2)]
pub fn p2(input: &Inp) -> usize {
    p2_inner(input)
}
