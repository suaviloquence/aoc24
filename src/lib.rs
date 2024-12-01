use std::collections::{BTreeMap, BTreeSet};

use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};

#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|x| {
            let (a, b) = x.split_once("   ").expect("a, b parsing x");
            (a.parse().expect("parsing a"), b.parse().expect("parsing b"))
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn d11(pairs: &Vec<(usize, usize)>) -> usize {
    let (mut a, mut b): (Vec<_>, Vec<_>) = pairs.iter().copied().unzip();
    a.sort_unstable();
    b.sort_unstable();

    a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2)]
pub fn d12(pairs: &Vec<(usize, usize)>) -> usize {
    let (a, b): (Vec<_>, Vec<_>) = pairs.iter().copied().unzip();
    let mut f: BTreeMap<_, usize> = BTreeMap::new();
    for itm in b {
        *f.entry(itm).or_default() += 1;
    }

    a.into_iter().map(|x| x * *f.entry(x).or_default()).sum()
}

aoc_lib! { year = 2024 }
