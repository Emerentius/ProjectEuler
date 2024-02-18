use std::collections::HashMap;

// faster, non cryptographically secure hasher for the hashmap
use fxhash::FxBuildHasher;

const N: usize = 100_000_000;

type BitPattern = u32;
// sentinel value for 0-digit long suffix, otherwise 0 and empty are the same
// when shifted left, the 1 will overflow and the rest will be the real bit pattern
const EMPTY: BitPattern = 1 << 31;
fn main() {
    // for all primes, for each bit-pattern in the n least significant bits, count how often it occurs
    // and how often 1 is the next bit.
    let mut bit_pattern_counts: HashMap<BitPattern, (u32, u32), FxBuildHasher> = HashMap::default();
    for mut p in primal::Primes::all().take_while(|&p| p < N) {
        let mut bit_pattern: u32 = EMPTY; // binary
        while p != 0 {
            let (n_pattern, n_pattern_next_1) =
                bit_pattern_counts.entry(bit_pattern).or_insert((0, 0));
            *n_pattern += 1;
            let lowest_bit = (p & 1) as u32;
            *n_pattern_next_1 += lowest_bit;
            bit_pattern = (bit_pattern << 1) + lowest_bit;

            p /= 2;
        }
    }
    let n_primes_total = bit_pattern_counts[&EMPTY].0;

    let expected_value = bit_pattern_counts
        .into_iter()
        .map(|(_, (n_primes_pattern, n_primes_next_1))| {
            let best_guess_count =
                std::cmp::max(n_primes_next_1, n_primes_pattern - n_primes_next_1);
            // probability_of_previous_pattern * best_guess_prob
            best_guess_count as f64 / n_primes_total as f64
        })
        .sum::<f64>();
    println!("{expected_value:.8}");
}
