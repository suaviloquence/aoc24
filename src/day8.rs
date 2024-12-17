use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::grid::{Grid, Vector2};

type Inp = Grid;

fn parse_inner(input: &str) -> Inp {
    Grid::parse(input)
}

type P1 = usize;
fn p1_inner(input: &Inp) -> P1 {
    let mut antinodes = BTreeSet::new();
    for (i, &c) in input.enumerate().filter(|(_, c)| **c != '.') {
        for (j, _) in input.enumerate().filter(|&(j, &x)| j != i && x == c) {
            let dist = j.i() - i.i();

            let a = i.wrapping_sub_i(dist);
            if input.contains(a) {
                antinodes.insert(a);
            }
        }
    }

    antinodes.len()
}

type P2 = usize;
fn p2_inner(input: &Inp) -> P2 {
    let mut antinodes = BTreeSet::new();
    for (i, &c) in input.enumerate().filter(|(_, c)| **c != '.') {
        for (j, _) in input.enumerate().filter(|&(j, &x)| j != i && x == c) {
            let dist = j.i() - i.i();
            let (m, d) = dist.to_sgn();
            let dist = Vector2::from_sgn(m.simplify(), d);

            let mut a = i;
            while input.contains(a) {
                antinodes.insert(a);
                a = a.wrapping_sub_i(dist);
            }
        }
    }

    antinodes.len()
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Inp {
    parse_inner(input)
}

#[aoc(day8, part1)]
pub fn p1(input: &Inp) -> usize {
    p1_inner(input)
}

#[aoc(day8, part2)]
pub fn p2(input: &Inp) -> usize {
    p2_inner(input)
}

const INP: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[test]
fn test() {
    let x = parse(INP);
    panic!("{}", p2(&x));
}
