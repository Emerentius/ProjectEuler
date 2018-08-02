#![feature(test)]
extern crate test;

extern crate primal;
use primal::is_prime;

const LIMIT: u64 = 100_000_000_000_000;

// generate right truncatable harshad numbers (depth first)
// sum all strong, right truncatable harshad primes (srt_hp)
fn find_srt_hp(num: u64, digit_sum: u64, trunc_is_strong: bool) -> u64 {
    let mut sum = if trunc_is_strong && is_prime(num) { num } else { 0 };

    if num * 10 < LIMIT // next number won't be above limit
    && num % digit_sum == 0 // is harshad
    {
        let is_strong_harshad = is_prime( num / digit_sum );
        for digit in 0..9+1 {
            sum += find_srt_hp(num * 10 + digit, digit_sum + digit, is_strong_harshad);
        }
    }
    sum
}

fn main() {
    let sum = (1..9+1)
        .map(|num| find_srt_hp(num, num, false) )
        .fold(0, |sum, partial_sum| sum + partial_sum);
    println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
