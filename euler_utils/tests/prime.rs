extern crate euler_utils;
use euler_utils::prime;

#[test]
fn miller_rabin() {
    let max_prime : u64 = 100_000;
    let primes = prime::sieve( max_prime as usize);

    // false positives
    let mut it = primes.iter();
    let mut prime = *(it.next().unwrap());
    for num in 2..max_prime+1 {
        if num == prime {
            if let Some(pr) = it.next() {
                prime = *pr;
            }
            continue
        }
        if prime::is_prime(num) { panic!("false positive: {}", num) }
    }

    // false negatives
    for &prime in &primes {
        if !prime::is_prime(prime) { panic!("false negative: {}", prime) }
    }
}

#[test]
fn miller_rabin_overflow_squares() {
    let max : u64 = 65535;//1_000_000;
    for n in 1..max {
        // this is  a check for overflows within is_prime(), the false positive
        // is just there to use the value, but might also come in handy
        if prime::is_prime(n*n) { panic!("false positive: {}", n*n) }
    }
}

#[test]
fn eratosthenes_equals_atkin() {
    let max_prime = 1_000_000;
    let primes_erat = prime::erat_sieve(max_prime);
    let primes_atkin = prime::atkin_sieve(max_prime);
    assert!(primes_erat == primes_atkin);
}

#[test]
fn primes_iter() {
    let max_prime = 1_000_000;
    let primes_iter = prime::prime_iter(max_prime);
    let primes = prime::atkin_sieve(max_prime);
    for (&pr1, pr2) in primes.iter().zip(primes_iter) {
        if pr1 != pr2 { panic!("{} != {}", pr1, pr2) }
        assert!(pr1 == pr2);
    }
}
