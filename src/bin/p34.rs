use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

fn solve() -> usize {
    let factorial = |n| (1..=n).product::<usize>() as usize;

    let selffact = |n: usize| {
        n.to_string()
            .chars()
            .map(|e| factorial(e.to_digit(10).unwrap() as usize))
            .sum::<usize>()
            == n
    };

    // LMAOOOOO GET FUCKED MATHRDS KAPPA KAPPA YIIIIHAWWW

    (3..=1_000_000)
        .into_par_iter()
        .filter(|&n| selffact(n))
        .sum()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
