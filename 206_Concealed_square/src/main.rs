#![feature(test)]
extern crate test;

fn main() {
	let max : u64 = 1389026623; // sqrt(1929394959697989900)
	let min : u64 = 1010101010; // sqrt(1020304050607080900)

	// number is guaranteed to be divisible by 10, also by 30 or 70
	'int:for n in (min..max).step_by(10).filter(|n| n % 70 == 0 && n % 30 == 0) {
		let mut num = n*n/100;
		let mut fixed_digit = 9;
		while num != 0 {
			if num % 10 != fixed_digit { continue 'int }
			num /= 100;
			fixed_digit -= 1;
		}
		println!("Result is {}", n);
		break;
	}
}

#[bench]
fn bench(b: &mut test::Bencher) {
	b.iter(|| main())
}
