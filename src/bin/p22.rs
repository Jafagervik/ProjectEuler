use itertools::Itertools;
use rayon::prelude::*;

fn solve() -> usize {
    let names: Vec<String> = include_str!("../p22.txt")
        .trim()
        .split(',')
        .map(|n| n.replace("\"", "").parse().unwrap())
        .sorted()
        .collect();

    let charpos = |c: char| (c as u8 - b'A' + 1) as usize;

    let namescore = |n: String| n.chars().map(charpos).sum::<usize>();

    names
        .into_par_iter()
        .enumerate()
        .map(|(idx, name)| (idx + 1) * namescore(name))
        .sum::<usize>()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
