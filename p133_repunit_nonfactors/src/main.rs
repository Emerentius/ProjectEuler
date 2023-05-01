use euler_utils::num::pow_mod;
use euler_utils::prime::DivisorExt;

fn main() {
    let sieve = primal::Sieve::new(100_000);

    // Same method as in p129, p130, except this one doesn't need a totient sieve, as it's
    // restricted to primes
    let a = |n: u64| {
        debug_assert!(primal::is_prime(n));
        debug_assert!(n > 3);
        // ϕ(9p) = ϕ(9) * ϕ(p) = 6 * (p-1) for p prime and p > 3
        let totient = (n - 1) * 6;
        sieve
            .divisors(totient as usize)
            .unwrap()
            .into_iter()
            .find(|&k| pow_mod(10, k as u32, (9 * n) as u64) == 1)
            .unwrap()
    };

    let non_factor_primes = sieve
        .primes_from(7)
        .take_while(|&pr| pr < 100_000)
        .filter(|&p| {
            !sieve
                .factor(a(p as u64))
                .unwrap()
                .into_iter()
                .all(|(prime_factor, _)| prime_factor == 2 || prime_factor == 5)
        });

    println!("{}", 2 + 3 + 5 + non_factor_primes.sum::<usize>());
}
