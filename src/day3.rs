use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Inp = String;

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Inp {
    input.to_string()
}

#[aoc(day3, part1)]
pub fn p1(input: &Inp) -> usize {
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
pub fn p2(input: &Inp) -> usize {
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
