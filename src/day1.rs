use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Inp = Vec<(usize, usize)>;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Inp {
    input
        .lines()
        .map(|x| {
            let (a, b) = x.split_once("   ").expect("a, b parsing x");
            (a.parse().expect("parsing a"), b.parse().expect("parsing b"))
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn p1(input: &Inp) -> usize {
    let (mut a, mut b): (Vec<_>, Vec<_>) = input.iter().copied().unzip();
    a.sort_unstable();
    b.sort_unstable();

    a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2)]
pub fn p2(input: &Inp) -> usize {
    let (a, b): (Vec<_>, Vec<_>) = input.iter().copied().unzip();
    let mut f: BTreeMap<_, usize> = BTreeMap::new();
    for itm in b {
        *f.entry(itm).or_default() += 1;
    }

    a.into_iter().map(|x| x * *f.entry(x).or_default()).sum()
}
