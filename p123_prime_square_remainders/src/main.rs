#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime;

fn main() {
    let primes = prime::sieve(1_000_000);
    // only every second prime
    for (i,&prime) in primes.iter().enumerate().filter(|&(i,_)| i % 2 == 0) {
        let rest = 2*(i as u64+1)*prime; // n < prime
        if rest > 10_000_000_000 {
            println!("{}: {}", i+1, prime);
            break
        }
    }
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}

#[bench]
fn no_sieve (b: &mut test::Bencher) {
    let primes = prime::sieve(1_000_000);
    b.iter(|| {
        for (i,&prime) in primes.iter().enumerate().filter(|&(i,_)| i % 2 == 0) {
            let rest = 2*(i as u64+1)*prime; // n < prime
            if rest > 10_000_000_000 {
                println!("{}: {}", i+1, prime);
                break
            }
        }
    })
}

#[bench]
fn primality_check (b: &mut test::Bencher) {
    b.iter(|| {
        let mut count = 1;
        for n in (3..).step_by(2) {
            if !prime::is_prime(n) { continue }
            count += 1;
            if count % 2 != 0
            && 2*count*n > 10_000_000_000 {
                println!("{}: {}", count, n);
                break
            }
        }
    })
}
