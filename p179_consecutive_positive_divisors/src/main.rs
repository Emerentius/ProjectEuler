extern crate primal;

fn main() {
    let primes = primal::Sieve::new(10usize.pow(7)/2);
    let mut n_divisors_last = 0;
    let mut count = 0;
    for n in 2..10_000_000 {
        if n % 1_000_000 == 0 { println!("{}", n) }
        let factors = primes.factor(n).unwrap();
        let n_divisors = factors.into_iter()
            .fold(1, |prod, (_,occ)| prod * (occ+1));
        if n_divisors == n_divisors_last { count += 1 }
        n_divisors_last = n_divisors;
    }
    println!("{}", count);
}
