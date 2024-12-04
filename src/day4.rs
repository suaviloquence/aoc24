use aoc_runner_derive::{aoc, aoc_generator};

type Inp = Vec<Vec<char>>;

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[aoc(day4, part1)]
pub fn p1(input: &Inp) -> usize {
    let mut n = 0;

    let y = 0..input.len() as isize;
    for i in y.clone() {
        let i = i as usize;
        let x = 0..input[i].len() as isize;
        for j in x.clone() {
            let j = j as usize;
            if input[i][j] != 'X' {
                continue;
            }

            if x.contains(&(j as isize + 3)) {
                if input[i][j + 1] == 'M' && input[i][j + 2] == 'A' && input[i][j + 3] == 'S' {
                    n += 1;
                }
            }

            if x.contains(&(j as isize - 3)) {
                if input[i][j - 1] == 'M' && input[i][j - 2] == 'A' && input[i][j - 3] == 'S' {
                    n += 1;
                }
            }

            if y.contains(&(i as isize - 3)) {
                if input[i - 1][j] == 'M' && input[i - 2][j] == 'A' && input[i - 3][j] == 'S' {
                    n += 1;
                }
            }

            if y.contains(&(i as isize + 3)) {
                if input[i + 1][j] == 'M' && input[i + 2][j] == 'A' && input[i + 3][j] == 'S' {
                    n += 1;
                }
            }

            if y.contains(&(i as isize + 3)) && x.contains(&(j as isize + 3)) {
                if input[i + 1][j + 1] == 'M'
                    && input[i + 2][j + 2] == 'A'
                    && input[i + 3][j + 3] == 'S'
                {
                    n += 1;
                }
            }

            if y.contains(&(i as isize - 3)) && x.contains(&(j as isize + 3)) {
                if input[i - 1][j + 1] == 'M'
                    && input[i - 2][j + 2] == 'A'
                    && input[i - 3][j + 3] == 'S'
                {
                    n += 1;
                }
            }

            if y.contains(&(i as isize - 3)) && x.contains(&(j as isize - 3)) {
                if input[i - 1][j - 1] == 'M'
                    && input[i - 2][j - 2] == 'A'
                    && input[i - 3][j - 3] == 'S'
                {
                    n += 1;
                }
            }

            if y.contains(&(i as isize + 3)) && x.contains(&(j as isize - 3)) {
                if input[i + 1][j - 1] == 'M'
                    && input[i + 2][j - 2] == 'A'
                    && input[i + 3][j - 3] == 'S'
                {
                    n += 1;
                }
            }
        }
    }

    n
}

#[aoc(day4, part2)]
pub fn p2(input: &Inp) -> usize {
    let mut n = 0;

    let y = 0..input.len() as isize;
    for i in y.clone() {
        let i = i as usize;
        let x = 0..input[i].len() as isize;
        for j in x.clone() {
            let j = j as usize;
            if input[i][j] != 'A' {
                continue;
            }

            if i < input.len() - 1 && i > 0 && j < input.len() - 1 && j > 0 {
                let left = (input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M')
                    || (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S');
                let right = (input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M')
                    || (input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S');

                if left && right {
                    n += 1;
                }
            }
        }
    }

    n
}
