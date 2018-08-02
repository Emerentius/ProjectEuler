#![feature(test)]
extern crate test;

const N: usize = 50;

fn main() {
    let mut possibilities = [1u64; N+1];
    for n_squares in 1..possibilities.len() {
        // sum of (up to) last 4 nums
        possibilities[n_squares] = (&possibilities[n_squares.saturating_sub(4)..n_squares])
            .iter()
            .sum();
    }
    println!("{}", possibilities.last().unwrap());
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
