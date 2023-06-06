use itertools::Itertools;
use rayon::prelude::*;

fn get_nums() -> Vec<u128> {
    include_str!("../p13.txt")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .take(20)
                .collect::<String>()
                .parse::<u128>()
                .unwrap()
        })
        .collect()
}

fn solve() -> String {
    let nums = get_nums();

    nums.iter()
        .sum::<u128>()
        .to_string()
        .chars()
        .take(10)
        .collect()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
