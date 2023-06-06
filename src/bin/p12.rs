use rayon::prelude::*;

/// NOTE: SLOW AF
fn solve() -> usize {
    let divs = |x| (1..=x).into_par_iter().filter(|i| x % i == 0).count() as usize;

    let mut i: usize = 7;
    let mut val: usize = 28;

    loop {
        if divs(val) > 500 {
            return val;
        }

        i += 1;
        val = (1..=i).into_par_iter().sum::<usize>();
    }
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
