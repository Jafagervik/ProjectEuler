use itertools::Itertools;
use rayon::prelude::*;

// fn solve() -> u64 {
//     let n: u64 = 600851475143;
//     let sqrt_n = (n as f64).sqrt() as u64;
//
//     let is_prime = |x| (2..=x).into_par_iter().all(|e| x % e != 0);
//
//     (1..=sqrt_n).rev().find(|&e| is_prime(e)).unwrap() as u64
// }

fn solve() -> u64 {
    let n: u64 = 600851475143;
    let sqrt_n = (n as f64).sqrt() as u64;

    let is_factor = |x| {
        n % x == 0
            && (2..=(x as f64).sqrt() as u64)
                .into_par_iter()
                .all(|e| x % e != 0)
    };

    (1..=sqrt_n).rev().find(|&e| is_factor(e)).unwrap()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
