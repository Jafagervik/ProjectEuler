use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

fn get_permutations(end: u32) -> Vec<Vec<u32>> {
    let numbers: Vec<u32> = (0..=end).collect();
    let permutations = numbers.iter().permutations(numbers.len());
    permutations.map(|p| p.iter().cloned().collect()).collect()
}

struct Node {
    val: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: u32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, left: Option<Box<Node>>, right: Option<Box<Node>>) {
        self.left = left;
        self.right = right;
    }

    /// DFS all paths and find max
    fn find_highest_val_tree(&self) -> usize {
        0
    }

    fn dfs(&self) -> Vec<usize> {
        vec![0]
    }
}

/// NOTE: pArse from string to tree

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<Vec<u32>> = s
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        Ok(Node::new(3))
    }
}

///
/// We can only choose old idx or the current one plus one
fn solve() -> usize {
    let s = include_str!("../p18.txt");

    let root = s.parse::<Node>();

    root.find_highest_val_tree()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
