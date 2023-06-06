use rayon::prelude::*;

fn solve() -> usize {
    let d = |n: usize| (1..n).into_par_iter().filter(|e| n % e == 0).sum::<usize>();
    let amb = |a, b| d(a) == b && d(b) == a && a != b;

    (1..10_000)
        .into_par_iter()
        .filter(|&a| (1..10_000).any(|b| amb(a, b)))
        .filter(|&e| e < 10_000)
        .sum::<usize>()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
