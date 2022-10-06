#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime;

fn main() {
    let max : u64 = 100_000_000;
    let primes = prime::sieve( (max/2) as usize);

    let mut count = 0;
    for (i,&pr1) in primes.iter().enumerate() {
        count += (&primes[i..]).iter()
            .take_while(|&pr2| pr1*pr2 < max)
            .count();
    }
    println!("{}", count);
}

#[bench]
fn bench(b:&mut test::Bencher) {
    b.iter(|| main())
}
