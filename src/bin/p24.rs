use itertools::Itertools;
use rayon::prelude::*;

fn solve() -> usize {
    (0..=9)
        .into_iter()
        .permutations(10)
        .collect::<Vec<Vec<usize>>>()
        .iter()
        .nth(1_000_000 - 1)
        .unwrap()
        .iter()
        .fold(0, |acc, &digit| acc * 10 + digit) as usize
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
