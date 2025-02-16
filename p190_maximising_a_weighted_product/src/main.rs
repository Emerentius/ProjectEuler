#![feature(test)]
extern crate test;
// As the sum of x_i is conserved, at the global maximum of P, all δP/δx_i must be equal
// or else you could lower one and raise another to increase P.
// δP/δx_i = (i / x_i) * P
// => i / x_i = j / x_j ∀ i,j ∈ [1, m]
// => x_i = i * x1
//
// Σ x_i = m
// (Σ_1^m i) * x1 = (m * (m+1)/2) * x1 = m
// => x1 = 2/(m+1)
// => x_i = 2*i/(m+1)
fn p(m: i32) -> f64 {
    let x1 = 2.0 / (m + 1) as f64;
    (1..=m).map(|i| (i as f64 * x1).powi(i)).product()
}

fn main() {
    let solution = (2..=15).map(p).map(|p| p as u64).sum::<u64>();
    println!("{solution}");
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
