use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::grid::{Grid, Vector2};

type Inp = Grid;

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Inp {
    Grid::parse(input)
}

#[aoc(day4, part1)]
pub fn p1(input: &Inp) -> usize {
    let dirs = [
        Vector2::UP,
        Vector2::DOWN,
        Vector2::LEFT,
        Vector2::RIGHT,
        Vector2(-1, -1),
        Vector2(-1, 1),
        Vector2(1, -1),
        Vector2(1, 1),
    ];

    input
        .enumerate()
        .filter(|(_, x)| **x == 'X')
        .map(|(v, _)| {
            dirs.iter()
                .filter(|&&d| {
                    input.get(v.wrapping_add_i(d)) == Some(&'M')
                        && input.get(v.wrapping_add_i(d * 2)) == Some(&'A')
                        && input.get(v.wrapping_add_i(d * 3)) == Some(&'S')
                })
                .count()
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn p2(input: &Inp) -> usize {
    let dirs = [Vector2(-1, 1), Vector2(1, 1)];

    input
        .enumerate()
        .filter(|(_, x)| **x == 'A')
        .filter(|(v, _)| {
            dirs.iter().all(|&d| {
                match (
                    input.get(v.wrapping_sub_i(d)),
                    input.get(v.wrapping_add_i(d)),
                ) {
                    (Some('M'), Some('S')) | (Some('S'), Some('M')) => true,
                    _ => false,
                }
            })
        })
        .count()
}
