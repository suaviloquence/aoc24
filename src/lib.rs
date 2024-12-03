use std::collections::{BTreeMap, BTreeSet};

use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};
use regex::Regex;

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

type Day2 = Vec<Vec<usize>>;

#[aoc_generator(day2)]
pub fn g2(input: &str) -> Day2 {
    input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn d21(input: &Day2) -> usize {
    input
        .iter()
        .filter(|x| {
            let mut increasing = true;
            let mut decreasing = true;
            let mut prev = None;
            for n in x.iter().copied() {
                if let Some(m) = prev {
                    match n as isize - m as isize {
                        1..=3 => {
                            decreasing = false;
                        }
                        -3..=-1 => {
                            increasing = false;
                        }
                        _ => return false,
                    }
                }
                prev = Some(n);
            }

            increasing || decreasing
        })
        .count()
}

#[aoc(day2, part2)]
pub fn d22(input: &Day2) -> usize {
    fn valid(x: &Vec<usize>) -> bool {
        let mut increasing = true;
        let mut decreasing = true;
        let mut prev = None;
        for n in x.iter().copied() {
            if let Some(m) = prev {
                match n as isize - m as isize {
                    1..=3 => {
                        decreasing = false;
                    }
                    -3..=-1 => {
                        increasing = false;
                    }
                    _ => return false,
                }
            }
            prev = Some(n);
        }

        increasing || decreasing
    }

    input
        .iter()
        .filter(|x| {
            valid(x)
                || (0..=x.len()).into_iter().any(|i| {
                    valid(
                        &x.iter()
                            .copied()
                            .enumerate()
                            .filter(|(j, _)| *j != i)
                            .map(|(_, x)| x)
                            .collect(),
                    )
                })
        })
        .count()
}

#[aoc_generator(day3)]
pub fn g3(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
pub fn d31<'a>(input: &String) -> usize {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    regex
        .captures_iter(input)
        .map(|x| {
            x.get(1).unwrap().as_str().parse::<usize>().unwrap()
                * x.get(2).unwrap().as_str().parse::<usize>().unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn d32(input: &String) -> usize {
    let mult = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut start = 0;

    let mut sum = 0;
    let mut enabled = true;

    while start < input.len() {
        let s = &input[start..];

        if enabled {
            match (s.find("don't()"), mult.captures(s)) {
                (Some(d), Some(m)) => {
                    if m.get(0).unwrap().start() < d {
                        sum += m.get(1).unwrap().as_str().parse::<usize>().unwrap()
                            * m.get(2).unwrap().as_str().parse::<usize>().unwrap();
                        start += m.get(0).unwrap().end();
                    } else {
                        enabled = false;
                        start += d + "don't()".len();
                    }
                }
                (Some(d), None) => {
                    enabled = false;
                    start += d + "don't()".len();
                }
                (None, Some(m)) => {
                    sum += m.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        * m.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    start += m.get(0).unwrap().end();
                }
                (None, None) => break,
            }
        } else {
            if let Some(pos) = s.find("do()") {
                enabled = true;
                start += pos + "do()".len();
            } else {
                break;
            }
        }
    }

    sum
}

aoc_lib! { year = 2024 }
