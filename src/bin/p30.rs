fn solve() -> usize {
    (2..1_000_000)
        .filter_map(|num| {
            let digit_sum: u32 = num
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap().pow(5))
                .sum();

            if digit_sum == num {
                Some(num as usize)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let result = solve();
    println!("{}", result); // Output: the sum of all numbers that can be written as the sum of fifth powers of their digits
}

