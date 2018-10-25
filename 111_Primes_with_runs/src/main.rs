#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime;

fn digits (mut num:u64) -> [u8;10] {
    let mut digits = [0;10];
    while num != 0 {
        digits[(num % 10) as usize] += 1;
        num /= 10;
    }
    digits
}

fn main() {
    let primes = prime::prime_iter(10_000_000_000 - 1);
    let mut sums = [[0;10];10]; // [dig][occ]

    const limit : u64 = 1_000_000_000;
    for prime in primes {//.skip_while(|&p| p < limit) {
        if prime < limit { continue }
        let digs = digits(prime);
        for (dig,&occ) in digs.iter().enumerate() {
            if occ >= 2 {
                sums[dig][occ as usize] += prime;
            }
        }
    }
    let mut sum = 0;
    for &sums_dig in sums.iter() {
        sum += *sums_dig.iter().rev().find(|&&s| s > 0).unwrap();
    }

    println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
