#![feature(test)]
extern crate test;
extern crate num;
use num::integer::gcd;
use std::f64::consts::E as e; // euler's number

fn main() {
    let mut sum = 0;
    for (n, n_fl) in (5..10_001).map(|n| (n, n as f64)) {
        // max solution, integer max is .floor() or .ceil()
        let mut k_fl = (n_fl / e).floor(); // .round() works, but I can't prove that it always does
        if n_fl / (k_fl+1.) * (k_fl/(k_fl+1.)).powi(k_fl as i32) > 1. {
            k_fl = k_fl + 1.
        }
        // ratio terminates if k_reduced is product of 2^x and 5^y only
        let mut k_reduced = k_fl as u64 / gcd(k_fl as u64, n);
        while k_reduced % 2 == 0 { k_reduced /= 2 }
        while k_reduced % 5 == 0 { k_reduced /= 5 }
        sum = if k_reduced == 1 { sum - n } else { sum + n };
    }
    println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
