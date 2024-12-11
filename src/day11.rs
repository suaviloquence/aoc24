use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Inp = Vec<usize>;

fn parse_inner(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

type P1 = usize;
fn p1_inner(input: &Inp) -> P1 {
    let mut prev = input.clone();
    let mut next;

    for _ in 0..25 {
        next = Vec::with_capacity(prev.len());
        for st in prev {
            if st == 0 {
                next.push(1);
            } else {
                let fmt = st.to_string();
                if fmt.len() % 2 == 0 {
                    next.push(fmt[..fmt.len() / 2].parse().unwrap());
                    next.push(fmt[fmt.len() / 2..].parse().unwrap());
                } else {
                    next.push(st * 2024);
                }
            }
        }

        prev = next;
    }

    prev.len()
}

type P2 = usize;
fn p2_inner(input: &Inp) -> P2 {
    let mut prev = input.iter().map(|&x| (x, 1)).collect::<BTreeMap<_, _>>();

    for i in 0..75 {
        println!("{i}");
        let mut next = BTreeMap::new();

        for (st, i) in prev {
            if st == 0 {
                *next.entry(1).or_default() += i;
            } else if ((st as f64).log10() + f64::EPSILON).ceil() as usize % 2 == 0 {
                let str = st.to_string();
                let a = str[..str.len() / 2].parse().unwrap();
                let b = str[str.len() / 2..].parse().unwrap();
                *next.entry(a).or_default() += i;
                *next.entry(b).or_default() += i;
            } else {
                *next.entry(st * 2024).or_default() += i;
            }
        }

        prev = next;
    }

    prev.into_values().sum()
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Inp {
    parse_inner(input)
}

#[aoc(day11, part1)]
pub fn p1(input: &Inp) -> usize {
    p1_inner(input)
}

#[aoc(day11, part2)]
pub fn p2(input: &Inp) -> usize {
    p2_inner(input)
}
