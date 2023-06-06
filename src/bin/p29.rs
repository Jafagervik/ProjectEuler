use itertools::Itertools;
use num_traits::Pow;

fn solve() -> usize {
    (2..=100)
        .flat_map(|a| (2..=100).map(move |b| (a, b)))
        .map(|(a, b)| (a as u32).pow(b as u32))
        .unique()
        .count()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
