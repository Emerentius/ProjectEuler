use euler_utils::num::pow_mod;
use euler_utils::prime::DivisorExt;
use euler_utils::prime::Phi32;
use num::Integer;

// The code to find a(n) is taken from p129, the rest is just a search of
// all non-prime natural numbers until 25 numbers matching the condition
// were found.
fn main() {
    let primes = primal::Sieve::new(1000);
    let totients = Phi32::new(300_000);

    let a_sequence = (2..)
        .filter(|n| 10.gcd(n) == 1 && !primal::is_prime(*n))
        .map(|n| {
            let totient = totients[(9 * n) as u32];

            let a_of_n = primes
                .divisors(9 * totient as usize)
                .unwrap()
                .into_iter()
                .find(|&k| pow_mod(10, k as u32, (9 * n) as u64) == 1)
                .unwrap();
            (n, a_of_n)
        });

    let sum: u64 = a_sequence
        .filter(|&(n, a)| (n - 1) % a as u64 == 0)
        .map(|(n, _)| n)
        .take(25)
        .sum();

    println!("{sum}");
}
