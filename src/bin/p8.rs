use itertools::Itertools;
use rayon::prelude::*;

fn solve() -> usize {
    let mut word = include_str!("../longword.txt").to_string();

    word.pop();

    word.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        .windows(13)
        .map(|w| w.iter().product())
        .max()
        .unwrap()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
