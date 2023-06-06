fn solve() -> u64 {
    let num: u128 = 2u128.pow(1000);
    println!("{}", num.to_string());

    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
