use itertools::Itertools;
use rayon::prelude::*;

fn solve() -> usize {
    let s: usize = (1..=100).map(|x: usize| x.pow(2)).sum();

    let s2 = ((1..=100).sum::<usize>().pow(2)) as usize;

    s2 - s
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
