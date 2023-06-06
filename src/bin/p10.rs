///
/// SIEVES ALGO FOR PRIMES
///
/// FOR EACH NUMBER, REMOVE ALL MULTIPLES
///
/// WORKS LIKE A CHARM
fn solve() -> usize {
    let n = 10000000;

    let mut ps: Vec<usize> = Vec::new();
    let mut sieve = vec![true; 2_500_000];

    for p in 2..sieve.len() {
        // If this number is not already set to false it's a prime
        if sieve[p] {
            ps.push(p);

            // return if we're at the end
            if ps.len() == n {
                break;
            }

            // Remove all multiples of curr number going forward
            (p..sieve.len()).step_by(p).for_each(|i| sieve[i] = false);
        }
    }

    ps.iter().filter(|&v| *v < 2_000_000).sum::<usize>()
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}
