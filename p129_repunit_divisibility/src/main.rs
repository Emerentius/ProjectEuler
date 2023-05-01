use euler_utils::num::pow_mod;
use euler_utils::prime::DivisorExt;
use euler_utils::prime::Phi32;
use num::Integer;

fn main() {
    // Repunit of length k:
    // (10^k - 1) / 9
    //
    // If it has to be divisible by `n`, we get
    // (10^k - 1) / 9 = 0 (mod n)
    // or 10^k - 1 = 0 (mod 9n)
    //      | Aside: (10^k - 1)/9 = n * SOME_INTEGER
    //               <=> 10^k - 1 = 9*n*SOME_INTEGER
    //               <=> 10^k - 1 = 0 (mod 9n)
    //
    // <=> 10^k = 1 (mod 9n)
    //
    // Euler's theorem gives us a candidate:
    // a^phi(9n) = 1 (mod 9n)
    // where phi(n) is the totient of n.
    // It requires that a is coprime to 9n, but we have that.
    //
    // Only problem is, a^φ(n) is not necessarily
    // the smallest exponent to achieve congruency with 1.
    // Still, the divisors of φ(n) give us the candidates that could be minimal.
    // So we can take the value of the totient function, factorize
    // it and then construct all the divisors and find the minimal one among them that
    // creates a power that is congruent to 1.
    //
    // There is a generalization of Euler's theorem which lowers the exponent
    // to the minimal number such that a^m = 1 (mod n) for any exponent.
    // It's called the carmichael function.
    // (https://en.wikipedia.org/wiki/Carmichael_function#Computing_%CE%BB(n)_with_Carmichael's_theorem)
    // While that lowers the amount of divisors to check, the carmichael function value is not necessarily
    // the solution we're looking for, because the minimal
    // exponent for a *specific* `a` can still be lower than the carmichael function λ(n).
    // For example: λ(13) = 12, but 10^6 = 1 (mod 13)

    let primes = primal::Sieve::new(1_000_000);
    let totients = Phi32::new(10_000_000);
    let target_limit: usize = 1_000_000;

    // starting with n > target_limit, because A(n) is always less than n,
    // because 10^φ(n) = 1 (mod n) always gives a solution with k < n due to φ(n) < n.
    for n in (target_limit..).filter(|n| 10.gcd(n) == 1) {
        let totient = totients[(9 * n) as u32];
        if (totient as usize) <= target_limit {
            continue;
        }

        let min_k = primes
            .divisors(totient as usize)
            .unwrap()
            .into_iter()
            .find(|&k| pow_mod(10, k as u32, (9 * n) as u64) == 1)
            .unwrap();
        if min_k > target_limit {
            println!("n = {n}, min_k = {min_k}");
            break;
        }
    }
}
