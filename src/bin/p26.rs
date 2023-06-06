use itertools::Itertools;
use rayon::prelude::*;

fn solve(n: usize) -> usize {
    let reps = |i: f64| {
        let s = format!("{}", 1f64 / i).to_string();

        let pattern = "sss".to_owned();

        let occurrences = 3;

        occurrences
    } as usize;

    (1..=n).map(|i| reps(i as f64)).position_max().unwrap() as usize
}

fn main() {
    let sol = solve(1000);

    println!("Solution: {:?}", sol);
}
