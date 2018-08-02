#![feature(step_by)]
extern crate int_sqrt; // my integer squareroot library
use int_sqrt::IntSqrt;

fn main() {
    let mut sigma_2 = vec![0; 64_000_000];
    for divisor in 1..64_000_000 {
        let div_squared = divisor * divisor;
        for divisible_num in (divisor..64_000_000).step_by(divisor) {
            sigma_2[divisible_num] += div_squared;
        }
    }

    println!("{}", sigma_2.iter()
        .enumerate()
        .filter(|&(_, &sigma2)| sigma2.is_square())
        .fold(0, |sum, (num, _)| sum + num)
    );
}
