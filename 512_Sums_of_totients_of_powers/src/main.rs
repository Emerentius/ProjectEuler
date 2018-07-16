#![feature(test)]
#![feature(step_by)] // for .step_by()
#![feature(iter_arith)] // for .sum()
extern crate test;
extern crate prime; // my prime library

const N: u32 = 500_000_000;

fn main() {
    let totients = prime::Phi32::new(N);
    let g: u64 = (1..N+1)
        .step_by(2)
        .map(|i| totients[i] as u64)
        .sum();
    println!("{}", g);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
