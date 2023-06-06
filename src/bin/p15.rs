use itertools::Itertools;
use rayon::prelude::*;

fn solve() -> usize {
    let m = 20;
    let n = 20;

    let mut mat: Vec<Vec<usize>> = vec![vec![1; n + 1]; m + 1];

    for i in 1..m + 1 {
        for j in 1..n + 1 {
            mat[i][j] = mat[i - 1][j] + mat[i][j - 1];
        }
    }

    mat[m][n]
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
