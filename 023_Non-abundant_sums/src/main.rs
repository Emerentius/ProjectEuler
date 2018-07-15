#![feature(test)]
extern crate test;

fn prop_sum (nr : i32) -> i32 {
	let mut proper_sum = 1;
	for i in 2..(nr as f64).sqrt() as i32 +1 {
		if nr%i == 0 {
			proper_sum += i;
			if i*i != nr {
				proper_sum += nr / i;
			}
		}
	};
	proper_sum
}

fn main() {
	// find abundant numbers
	let mut abundant_numbers = vec![];
	for i in (2..28123+1).filter(|&i| prop_sum(i) > i) {
		abundant_numbers.push(i)
	}

	println!("{} abundant numbers found.", abundant_numbers.len());
	let mut abundant_summable = [false; 28123+1];
	for (i, &ab_nr1) in abundant_numbers.iter().enumerate() {
		for &ab_nr2 in abundant_numbers[i..].iter() {
			let ab_sum = (ab_nr1 + ab_nr2) as usize;
			if ab_sum <= 28123+1 { abundant_summable[ab_sum - 1] = true }
		}
	}

	let sum = (1..28123+1).zip(abundant_summable.iter())
		.filter(|&(_, &is_abundant)| is_abundant)
		.fold(0, |sum, (num, _)| sum + num);
	println!("{}", sum);

}

#[bench]
fn bench(b: &mut test::Bencher) {
	b.iter(main)
}
