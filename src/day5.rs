use aoc_runner_derive::{aoc, aoc_generator};

type Inp = (Vec<(usize, usize)>, Vec<Vec<usize>>);

fn parse_inner(input: &str) -> Inp {
    let (a, b) = input.split_once("\n\n").unwrap();

    let a = a
        .lines()
        .map(|x| {
            let (c, d) = x.split_once("|").unwrap();
            (c.parse().unwrap(), d.parse().unwrap())
        })
        .collect();

    let b = b
        .lines()
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    (a, b)
}

type P1 = usize;
fn p1_inner((rules, orders): &Inp) -> P1 {
    orders
        .iter()
        .filter(|r| {
            r.iter().enumerate().all(|(i, x)| {
                rules
                    .iter()
                    .filter(|y| y.0 == *x)
                    .map(|x| x.1)
                    .all(|z| !r[..i].contains(&z))
            })
        })
        .map(|x| x[x.len() / 2])
        .sum()
}

type P2 = usize;
fn p2_inner((rules, orders): &Inp) -> P2 {
    let inc = orders.iter().filter(|r| {
        !r.iter().enumerate().all(|(i, x)| {
            rules
                .iter()
                .filter(|y| y.0 == *x)
                .map(|x| x.1)
                .all(|z| !r[..i].contains(&z))
        })
    });

    inc.map(|x| {
        let mut vec = x.iter().copied().collect::<Vec<_>>();
        let mut changed = true;

        while changed {
            changed = false;
            for i in 0..vec.len() {
                if let Some((j, _)) = rules
                    .iter()
                    .filter(|y| y.0 == vec[i])
                    .find_map(|(_, z)| vec[..i].iter().enumerate().find(|(_, j)| *j == z))
                {
                    let x = vec[i];
                    vec[i] = vec[j];
                    vec[j] = x;
                    changed = true;
                }
            }
        }

        vec
    })
    .map(|x| x[x.len() / 2])
    .sum()
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Inp {
    parse_inner(input)
}

#[aoc(day5, part1)]
pub fn p1(input: &Inp) -> usize {
    p1_inner(input)
}

#[aoc(day5, part2)]
pub fn p2(input: &Inp) -> usize {
    p2_inner(input)
}
