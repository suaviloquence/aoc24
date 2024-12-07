pub mod grid;

pub fn usize_lists(s: &str) -> Vec<Vec<usize>> {
    s.lines()
        .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect()
}

// pub fn usize_pairs(s: &str) -> Vec<Vec<usize>> {
//     s.lines()
//         .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
//         .collect()
// }
