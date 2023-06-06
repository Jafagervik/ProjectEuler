use itertools::Itertools;
use rayon::prelude::*;

fn solve() -> usize {
    let is_palindrome = |x: usize| {
        let rev: String = x.to_string().chars().rev().collect();
        x.to_string() == rev
    };

    (100..=999)
        .flat_map(|o| (100..=999).map(move |u| (u, o)))
        .rev()
        // .take(1)
        // .for_each(|(i, j)| println!("{} {}", i, j));
        .filter_map(|(i, j)| {
            // println!("{} {}", i, j);
            let product = i * j;
            if is_palindrome(product) {
                Some(product)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
