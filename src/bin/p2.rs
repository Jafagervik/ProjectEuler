// use itertools::Itertools;

fn solve() -> usize {
    let mut sum = 0;
    let (mut a, mut b) = (1, 2);

    while a <= 4_000_000 {
        if a % 2 == 0 {
            sum += a;
        }

        std::mem::swap(&mut a, &mut b);
        b += a;
    }

    sum
}

fn main() {
    let sol = solve();

    println!("Solution: \n{:?}", sol);
}
