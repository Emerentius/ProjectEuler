fn prop_sum (nr : i32) -> i32 {
	let mut proper_sum = 0;
	for i in 1..nr/2+1 {
		if nr%i == 0 { proper_sum += i }
	};
	proper_sum
}

fn main() {
	// find abundant numbers
	let mut abundant_numbers = vec![];
	for i in 1..28123+1 { if prop_sum(i) > i { abundant_numbers.push(i) } }

	println!("{} abundant numbers found.", abundant_numbers.len());
	let mut abundant_summable = [false; 28123+1];
	for &ab_nr1 in abundant_numbers.iter() {
		for &ab_nr2 in abundant_numbers.iter() {
			let ab_sum = (ab_nr1 + ab_nr2) as usize;
			if ab_sum <= 28123+1 { abundant_summable[ab_sum - 1] = true }
			// abundant_summable [ summable_number ] = true
		}
	}

	let mut sum = 0;
	for (num, &summable) in (1..28123+1).zip(abundant_summable.iter()) {
		if !summable { sum += num }
	}
	println!("{}", sum);

}
