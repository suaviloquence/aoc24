use aoc_runner_derive::{aoc, aoc_generator};

type Inp = Vec<Vec<usize>>;

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Inp {
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
pub fn p1(input: &Inp) -> usize {
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
pub fn p2(input: &Inp) -> usize {
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
