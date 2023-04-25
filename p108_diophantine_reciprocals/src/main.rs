use euler_utils::prime::DivisorExt;
use primal::Sieve;

fn main() {
    let sieve = Sieve::new(10_000);

    for n in 1u64.. {
        let Some(n_solutions) =
            // Great explanation of why this works: https://projecteuler.net/thread=108#2455
            sieve
                .n_divisors((n as usize).pow(2))
                .map(|n_div| (n_div + 1) / 2)
        else {
            // Only highly composite numbers are of interest.
            // if we can't factorise it, then it contains a large prime
            // and can't be the solution.
            continue;
        };
        let n_solutions2 = (sieve.n_divisors_pow(n as usize, 2).unwrap() + 1) / 2;
        assert_eq!(n_solutions, n_solutions2);

        if n_solutions > 1000 {
            println!("{}: {}", n, n_solutions);
            break;
        }
    }
}
