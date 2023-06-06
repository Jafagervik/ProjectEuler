use itertools::Itertools;
use rayon::prelude::*;

use std::cmp;
//
// fn solve() -> usize {
//     let next_even = |x| x / 2;
//     let next_odd = |x| 3 * x + 1;
//
//     let next = |x| {
//         if x % 2 == 0 {
//             next_even(x)
//         } else {
//             next_odd(x)
//         }
//     };
//
//     let mut longest = 0;
//     let mut res = 1;
//
//     for i in 1..1_000_000 {
//         let mut val = i;
//         let mut count = 0;
//
//         loop {
//             if val == 1 {
//                 break;
//             }
//
//             count += 1;
//             val = next(val);
//         }
//
//         if count > longest {
//             res = i;
//             longest = count;
//         }
//     }
//
//     res
// }

fn solve() -> usize {
    let next = |x| match x % 2 {
        0 => x / 2,
        _ => 3 * x + 1,
    };

    (1..1_000_000)
        .max_by_key(|&i| {
            let mut val = i;
            let mut count = 0;

            while val != 1 {
                count += 1;
                val = next(val);
            }

            count
        })
        .unwrap()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
