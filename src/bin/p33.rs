use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

fn solve() -> usize {
    for t in (10..=99) {
        for b in (10..=99) {
            if t >= b {
                break; // go to next
            }
        }
    }
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
