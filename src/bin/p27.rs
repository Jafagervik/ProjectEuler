use itertools::Itertools;
use rayon::prelude::*;

use num::BigUint;

fn solve() -> BigUint {
    (1..=1000)
        .into_par_iter()
        .map(|e: u32| e.pow(e as u32))
        .sum::<BigUint>()
        .to_string()
        .chars()
        .rev()
        .take(10)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<BigUint>()
        .unwrap()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
