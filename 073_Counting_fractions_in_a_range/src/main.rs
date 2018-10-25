#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime;

fn main() {
    let count = prime::FareySequence::new(12_000).skip_while(|&(n,d)| !(n == 1 && d == 3) )
        .take_while(|&(n,d)| !(n == 1 && d == 2))
        .count();
    println!("{}", count - 1); // 1/3 is counted, this corrects it
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
