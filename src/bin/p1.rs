use itertools::Itertools;

fn solve(e: usize) -> usize {
    (1..e).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

fn main() {
    let sol = solve(1000);

    println!("Solution: {:?}", sol);
}
