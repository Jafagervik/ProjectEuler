use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

fn solve(max: usize) -> usize {
    let abundant = |n: usize| (1..n).into_par_iter().filter(|e| n % e == 0).sum::<usize>() > n;

    let abundants: Vec<usize> = (1..max).into_par_iter().filter(|&n| abundant(n)).collect();

    let perm_sums: HashSet<usize> = abundants
        .iter()
        .cartesian_product(abundants.iter())
        .map(|(&a, &b)| a + b) // Calculate the sum of each permutation
        .collect();

    // all numbers from 1..28213 not in perm sums should be returned
    (1..max)
        .into_par_iter()
        .filter(|&n| !perm_sums.contains(&n))
        .sum()
}

fn main() {
    let sol = solve(28213);

    println!("Solution: {:?}", sol);
}
