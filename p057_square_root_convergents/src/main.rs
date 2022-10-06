extern crate num;
use num::rational;
use num::rational::BigRational;
use num::{BigInt, One, Zero};

fn cont_fraction ( iter : u64, first : bool ) -> BigRational {
	let one = || rational::BigRational::new(One::one(), One::one());

	let big_one : BigInt = One::one();
	let big_two : BigInt = big_one.clone() + big_one.clone();
	let two = || rational::BigRational::new(big_two, big_one);

	match iter {
		0 => rational::BigRational::new(Zero::zero(), One::one()),
		_ if first => (one() + one()/(two() + cont_fraction(iter-1, false))).reduced(),
		_  => (one()/(two() + cont_fraction(iter-1, false))),
	}
}

fn main() {
	let mut count = 0;
	for steps in 1..1001 {
		let ratio = cont_fraction(steps, true);
		/*if steps < 10 {
			println!("{}", ratio);
		}*/
		if ratio.numer().to_string().len() > ratio.denom().to_string().len() {
			count += 1;
		}
	}
	println!("{}", count);
}
