fn solve() -> usize {
    let fact100 = (1..=100).product::<u128>();

    fact100
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum::<u64>() as usize
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
