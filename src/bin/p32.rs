use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

static RES: &str = "123456789";

fn solve() -> u64 {
    let pandigital = |m: u64, n: u64| {
        format!("{}{}{}", m, n, m * n)
            .chars()
            .sorted()
            .collect::<String>()
            == RES
    };

    (1..=10_000)
        .flat_map(|i: u64| (1..=1000).map(move |j: u64| (i, j)))
        .filter_map(|(i, j)| {
            if i != j && pandigital(i, j) {
                Some(i * j)
            } else {
                None
            }
        })
        .collect::<HashSet<u64>>()
        .iter()
        .sum::<u64>()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
