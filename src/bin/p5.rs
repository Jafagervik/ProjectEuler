use itertools::Itertools;
use rayon::prelude::*;

fn solve() -> usize {
    (1..).find(|&e| (1..=20).all(|x| e % x == 0)).unwrap()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
