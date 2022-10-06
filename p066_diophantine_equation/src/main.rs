extern crate num;
//use num::rational;
//use num::rational::BigRational;
use num::integer::Integer;
use num::{BigInt, One};
use num::bigint::ToBigInt;
use std::cmp::max;
use std::mem;

struct Ratio (BigInt, BigInt);
impl Ratio {
	fn numer(&self) -> &BigInt { &(self.0) }
	fn denom(&self) -> &BigInt { &(self.1) }
}

fn next_convergent (ratio: &Ratio, prev_ratio: Ratio, a_n : &BigInt, b_n : &BigInt ) -> Ratio {
	// nth_convergent = p_n / q_n
	// p_n = b_n * p_n-1 + a_n * p_n-2
	// q_n = b_n * q_n-1 + a_n * q_n-2
	let numer = ratio.numer();
	let denom = ratio.denom();
	let prev_numer = prev_ratio.numer();
	let prev_denom = prev_ratio.denom();
	let next_numer = b_n * numer + a_n * prev_numer;
	let next_denom = b_n * denom + a_n * prev_denom;
	Ratio(next_numer, next_denom)
}

fn minimal_diophantine_x ( D : usize ) -> BigInt {
	// continued fraction = b_0 + a_1/(b_1 + a_2/b_2 + ... )
	let a_n = (D - 1).to_bigint().unwrap(); // n>= 1
	let b_n = 2.to_bigint().unwrap();	 // n>= 1, b_0 = 1
	
	let one : BigInt = One::one();
	let two : BigInt = 2.to_bigint().unwrap();
	let D = D.to_bigint().unwrap();
	
	let mut prev_convergent = Ratio(one.clone(), one.clone());
	let mut convergent = Ratio(&D+one, two);
	
	loop {
		{
			let x = convergent.numer();
			let y = convergent.denom();
			let divisor = x.gcd(y);
			let ref x = x / &divisor;
			let ref y = y / &divisor;
			let num = x*x - &D*y*y;
			println!("ratio: {}/{}", x,y);
			
			if num == One::one() { return x.clone() }
		}

		prev_convergent = next_convergent(&convergent, prev_convergent, &a_n, &b_n);
		mem::swap(&mut prev_convergent, &mut convergent);
	}
}

fn main() {
	let mut is_square = [false; 1000+1];
	for i in 0..32 { is_square[i*i] = true; } // 32*32 > 1000
	
	let mut max_x : BigInt = One::one();
	
	println!("{}", minimal_diophantine_x(7));
	/*
	for (num,&is_sq) in is_square.iter().enumerate() {
		if is_sq { continue; }
		if num % 1 == 0 { println!("num: {}", num); }
		
		let min_dioph = minimal_diophantine_x(num);
		
		
		if &min_dioph > &max_x { max_x = min_dioph }
		//max_x = max( max_x, min_dioph );
	}*/
	println!("x_max: {}", max_x);
}
