#![feature(test)]
extern crate test;

fn gauss_sum(n: u64) -> u64 { n*(n+1)/2 }

fn h(n: u64) -> u64 {
    (1..n/3+1)
        .map(|i| i * gauss_sum(n+1 - 3*i)) // 3*i == min triangle size
        .sum()
}

fn main() {
    let solution: u64 = (3..12345+1)
        .map(h)
        .sum();
    println!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
