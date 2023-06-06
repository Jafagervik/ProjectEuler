use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

fn solve() -> usize {
    let mut s = String::new();

    for i in 1..=1_000_000 {
        s.push_str(format!("{}", i).as_str());
    }

    s.chars().nth(1 - 1).unwrap().to_digit(10).unwrap() as usize
        * s.chars().nth(10 - 1).unwrap().to_digit(10).unwrap() as usize
        * s.chars().nth(100 - 1).unwrap().to_digit(10).unwrap() as usize
        * s.chars().nth(1000 - 1).unwrap().to_digit(10).unwrap() as usize
        * s.chars().nth(10000 - 1).unwrap().to_digit(10).unwrap() as usize
        * s.chars().nth(100000 - 1).unwrap().to_digit(10).unwrap() as usize
        * s.chars().nth(1000000 - 1).unwrap().to_digit(10).unwrap() as usize
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
