#![feature(test)]
extern crate my_combinatorics;
extern crate primal;

use my_combinatorics::PermutationsStreamIter;

fn count(min_num: u64, remaining_digits: &[u64]) -> u64 {
    if remaining_digits.len() == 0 { return 1 }

    let mut count_ = 0;
    let mut num = 0;
    for (i, digit) in remaining_digits.iter().enumerate() {
        num = num*10 + digit;
        if num > min_num && primal::is_prime(num) {
            count_ += count(num, &remaining_digits[i+1..]);
        }
    }
    count_
}

fn main() {
    let mut perm_iter = PermutationsStreamIter::new([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut count_ = 0;
    while let Some(perm) = perm_iter.streaming_next() {
        count_ += count(0, perm);
    }
    println!("{}", count_);
}

mod test {
    extern crate test;
    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| ::main())
    }
}
