#![feature(test)]
extern crate test;
extern crate prime;

fn main() {
    let totients = prime::Phi32::new(1_000_000);
    let n_elem = totients.iter().skip(1).fold(0, |sum, tot| sum + *tot as u64);
    println!("{}", n_elem);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
