use itertools::Itertools;
use rayon::prelude::*;

fn solve(amount: usize) -> usize {
    let coins: Vec<usize> = vec![1, 2, 5, 10, 20, 50, 100, 200];

    let mut combinations = vec![0; (amount + 1) as usize];
    combinations[0] = 1;

    coins.iter().for_each(|&coin| {
        (coin..=amount).for_each(|i| combinations[i] += combinations[(i - coin) as usize])
    });

    combinations[amount]
}

fn main() {
    let sol = solve(200);

    println!("Solution: {:?}", sol);
}
