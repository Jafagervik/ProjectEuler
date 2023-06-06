// use itertools::Itertools;
use rayon::prelude::*;
// use std::collections::HashMap;
//
// fn solve() -> usize {
//     let mut letter_counts: HashMap<String, usize> = HashMap::new();
//
//     letter_counts.insert("0".to_string(), 0);
//     letter_counts.insert("1".to_string(), 3);
//     letter_counts.insert("2".to_string(), 3);
//     letter_counts.insert("3".to_string(), 5);
//     letter_counts.insert("4".to_string(), 4);
//     letter_counts.insert("5".to_string(), 4);
//     letter_counts.insert("6".to_string(), 3);
//     letter_counts.insert("7".to_string(), 5);
//     letter_counts.insert("8".to_string(), 5);
//     letter_counts.insert("9".to_string(), 4);
//     letter_counts.insert("10".to_string(), 3);
//     letter_counts.insert("11".to_string(), 6);
//     letter_counts.insert("12".to_string(), 6);
//     letter_counts.insert("13".to_string(), 8);
//     letter_counts.insert("14".to_string(), 8);
//     letter_counts.insert("15".to_string(), 7);
//     letter_counts.insert("16".to_string(), 7);
//     letter_counts.insert("17".to_string(), 9);
//     letter_counts.insert("18".to_string(), 8);
//     letter_counts.insert("19".to_string(), 8);
//     letter_counts.insert("20".to_string(), 6);
//     letter_counts.insert("30".to_string(), 6);
//     letter_counts.insert("40".to_string(), 5);
//     letter_counts.insert("50".to_string(), 5);
//     letter_counts.insert("60".to_string(), 5);
//     letter_counts.insert("70".to_string(), 7);
//     letter_counts.insert("80".to_string(), 6);
//     letter_counts.insert("90".to_string(), 6);
//
//     let one_thousand = 11;
//     let hundred_and = 10;
//
//     (1..1000)
//         .map(|number| {
//             let mut n = number;
//             let mut count = 0;
//
//             if n >= 100 {
//                 count += letter_counts[&n.to_string()[..1].to_string()];
//
//                 if n % 100 != 0 {
//                     count += hundred_and;
//                 }
//
//                 n %= 100;
//             }
//
//             if n >= 20 {
//                 count += letter_counts[&format!("{}0", n / 10)];
//                 n %= 10;
//             }
//
//             let mut cont = true;
//             if n >= 10 {
//                 count += letter_counts[&n.to_string()[..2].to_string()];
//                 cont = false;
//             }
//
//             if n > 0 && cont {
//                 count += letter_counts[&n.to_string()[..1].to_string()];
//             }
//
//             count
//         })
//         .sum::<usize>()
//         + one_thousand
// }
//
// fn main() {
//     let sol = solve();
//
//     println!("Solution: {:?}", sol);
// }
//
//

use std::collections::HashMap;

use rayon::prelude::IntoParallelIterator;

fn solve() -> usize {
    let mut letter_counts: HashMap<usize, usize> = HashMap::new();

    letter_counts.insert(0, 0);
    letter_counts.insert(1, 3);
    letter_counts.insert(2, 3);
    letter_counts.insert(3, 5);
    letter_counts.insert(4, 4);
    letter_counts.insert(5, 4);
    letter_counts.insert(6, 3);
    letter_counts.insert(7, 5);
    letter_counts.insert(8, 5);
    letter_counts.insert(9, 4);
    letter_counts.insert(10, 3);
    letter_counts.insert(11, 6);
    letter_counts.insert(12, 6);
    letter_counts.insert(13, 8);
    letter_counts.insert(14, 8);
    letter_counts.insert(15, 7);
    letter_counts.insert(16, 7);
    letter_counts.insert(17, 9);
    letter_counts.insert(18, 8);
    letter_counts.insert(19, 8);
    letter_counts.insert(20, 6);
    letter_counts.insert(30, 6);
    letter_counts.insert(40, 5);
    letter_counts.insert(50, 5);
    letter_counts.insert(60, 5);
    letter_counts.insert(70, 7);
    letter_counts.insert(80, 6);
    letter_counts.insert(90, 6);
    letter_counts.insert(100, 7);

    let and = 3;

    (1..1000)
        .into_par_iter()
        .map(|number| {
            let mut n = number;
            let mut count = 0;

            if n >= 100 {
                count += letter_counts[&(n / 100)]; // hundreds
                count += letter_counts[&100]; // hundred

                if n % 100 != 0 {
                    count += and; // and
                }

                n %= 100;
            }

            if n > 0 {
                // under 20 or any of the tys
                if n <= 20 || n % 10 == 0 {
                    count += letter_counts[&n];
                } else {
                    // ...ty + one|two...
                    count += letter_counts[&(n / 10 * 10)];
                    count += letter_counts[&(n % 10)];
                }
            }

            count
        })
        .sum::<usize>()
        + 11 // one thousand
}

fn main() {
    let result = solve();
    println!("Total number of letters used: {}", result);
}
