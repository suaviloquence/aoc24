use aoc_runner_derive::{aoc, aoc_generator};

type Inp = Vec<(usize, Vec<usize>)>;

fn parse_inner(input: &str) -> Inp {
    input
        .lines()
        .map(|x| {
            let (x, y) = x.split_once(": ").unwrap();
            let x = x.parse().unwrap();

            let y = y.split_whitespace().map(|x| x.parse().unwrap()).collect();

            (x, y)
        })
        .collect()
}

type P1 = usize;
fn p1_inner(input: &Inp) -> P1 {
    fn makes(v: usize, x: usize, mut i: impl Iterator<Item = usize> + Clone) -> bool {
        match i.next() {
            Some(y) => makes(v, x + y, i.clone()) || makes(v, x * y, i.clone()),
            None => v == x,
        }
    }
    input
        .iter()
        .filter(|(v, x)| {
            let mut i = x.iter().copied();
            let x = i.next().unwrap();
            makes(*v, x, i)
        })
        .map(|(v, _)| v)
        .sum()
}

type P2 = usize;
fn p2_inner(input: &Inp) -> P2 {
    fn makes(v: usize, x: usize, mut i: impl Iterator<Item = usize> + Clone) -> bool {
        match i.next() {
            Some(y) => {
                makes(v, x + y, i.clone())
                    || makes(v, x * y, i.clone())
                    || makes(
                        v,
                        // for actual aoc i did the format!("{x}{y}").parse().unwrap()
                        // but this is better
                        x * 10usize.pow(((y as f64).log10() + f64::EPSILON).ceil() as u32) + y,
                        i.clone(),
                    )
            }
            None => v == x,
        }
    }
    input
        .iter()
        .filter(|(v, x)| {
            let mut i = x.iter().copied();
            let x = i.next().unwrap();
            makes(*v, x, i)
        })
        .map(|(v, _)| v)
        .sum()
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Inp {
    parse_inner(input)
}

#[aoc(day7, part1)]
pub fn p1(input: &Inp) -> usize {
    p1_inner(input)
}

#[aoc(day7, part2)]
pub fn p2(input: &Inp) -> usize {
    p2_inner(input)
}

#[test]
fn test() {
    let t = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    panic!("{:?}", p2(&parse(t)))
}
