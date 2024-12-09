use aoc_runner_derive::{aoc, aoc_generator};

type Inp = Vec<(bool, u8, usize)>;

fn parse_inner(input: &str) -> Inp {
    input
        .chars()
        .enumerate()
        .map(|(i, x)| (i % 2 == 0, x.to_digit(10).unwrap() as u8, i / 2))
        .collect()
}

type P1 = usize;
fn p1_inner(input: &Inp) -> P1 {
    let mut input = input
        .iter()
        .copied()
        .flat_map(|(i, j, k)| (0..j).map(move |_| (i, k)))
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut j = input.len() - 1;

    while j > i {
        while input[i].0 {
            i += 1;
            if i == input.len() {
                break;
            }
        }

        while !input[j].0 {
            if j == 0 {
                break;
            }

            j -= 1;
        }

        let x = input[i];
        input[i] = input[j];
        input[j] = x;

        if j == 0 {
            break;
        }

        j -= 1;
        i += 1;
    }

    input
        .into_iter()
        .filter(|x| x.0)
        .enumerate()
        .map(|(i, (_, j))| i * j)
        .sum()
}

type P2 = usize;
fn p2_inner(input: &Inp) -> P2 {
    let mut input = input
        .iter()
        .copied()
        .flat_map(|(i, j, k)| (0..j).map(move |_| i.then_some(k)))
        .collect::<Vec<_>>();

    let mut j = input.len() - 1;

    while (0..input.len()).contains(&j) {
        while input[j].is_none() {
            if j == 0 {
                break;
            }

            j -= 1;
        }
        let Some(len) = (0..j).find(|&x| input[j - x] != input[j]) else {
            break;
        };

        let Some(i) = (0..j + 1 - len).find(|&i| input[i..i + len].iter().all(Option::is_none))
        else {
            j -= len;
            continue;
        };

        for n in 0..len {
            let x = input[i + n];
            input[i + n] = input[j - n];
            input[j - n] = x;
        }

        j -= len;
    }

    input
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(move |j| (i, j)))
        .map(|(i, j)| i * j)
        .sum()
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Inp {
    parse_inner(input)
}

#[aoc(day9, part1)]
pub fn p1(input: &Inp) -> usize {
    p1_inner(input)
}

#[aoc(day9, part2)]
pub fn p2(input: &Inp) -> usize {
    p2_inner(input)
}

#[test]
fn test() {
    panic!("{}", p2(&parse("2333133121414131402")))
}
