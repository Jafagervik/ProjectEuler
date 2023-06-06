use itertools::Itertools;
use rayon::prelude::*;

use num::traits::FromPrimitive;
use num::BigUint;

fn solve() -> u128 {
    let mut a: BigUint = FromPrimitive::from_u32(1).unwrap();
    let mut b: BigUint = FromPrimitive::from_u32(1).unwrap();
    let mut count = 2;

    loop {
        let temp = a.clone() + &b;
        a = b.clone();
        b = temp;
        count += 1;

        if b.to_string().len() == 1000 {
            break;
        }
    }

    count
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
