fn solve() -> usize {
    let palin = |s: String| s.chars().eq(s.chars().rev());

    let twoandten = |n: usize| palin(n.to_string()) && palin(format!("{:b}", n));

    (1..1_000_000).filter(|&e| twoandten(e)).sum()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
