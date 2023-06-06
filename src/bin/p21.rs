use itertools::Itertools;
use rayon::prelude::*;

fn solve() -> usize {
    let d = |n| (1..n).filter(|e| n % e == 0).sum::<usize>();
    let amb = |a, b| d(a) == d(b) && a != b;

    let ds: Vec<usize> = (1..10_000).into_par_iter().map(|n| d(n)).collect();

    ds.iter()
        .filter(|&e| ds.iter().find(|&a| d(a) == e).unwrap())
        .sum() as usize
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
