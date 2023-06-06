use itertools::Itertools;
use rayon::prelude::*;

fn solve(n: usize) -> usize {
    let mut dim = 3;
    let mut distance = 2;
    let mut sum: usize = 1;

    while dim <= n {
        // get the 4 nums in the corners
        sum += (0..4).map(|i| i * distance + sum).sum::<usize>();

        distance += 2;
        dim += 2 // 1x1 to 3x3 to 5x5
    }

    sum
}

fn main() {
    let sol = solve(1001);

    println!("Solution: {:?}", sol);
}
