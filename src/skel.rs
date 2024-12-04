use aoc_runner_derive::{aoc, aoc_generator};

type Inp = String;

fn parse_inner(input: &str) -> Inp {
    input.to_string()
}

type P1 = usize;
fn p1_inner(input: &Inp) -> P1 {
    todo!()
}

type P2 = usize;
fn p2_inner(input: &Inp) -> P2 {
    todo!()
}

#[aoc_generator(day{{N}})]
pub fn parse(input: &str) -> Inp {
    parse_inner(input)
}

#[aoc(day{{N}}, part1)]
pub fn p1(input: &Inp) -> usize {
    p1_inner(input)
}

#[aoc(day{{N}}, part2)]
pub fn p2(input: &Inp) -> usize {
    p2_inner(input)
}
