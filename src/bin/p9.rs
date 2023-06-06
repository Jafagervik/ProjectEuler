fn solve() -> usize {
    let ispyt = |a: usize, b: usize, c: usize| a.pow(2) + b.pow(2) == c.pow(2);

    (1..=500)
        .flat_map(|i| (1..=500).flat_map(move |j| (1..=500).map(move |k: usize| (k, j, i))))
        .find_map(|(a, b, c)| {
            if ispyt(a, b, c) && a + b + c == 1000 {
                Some(a * b * c)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
