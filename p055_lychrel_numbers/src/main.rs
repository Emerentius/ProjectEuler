#![feature(test)]
extern crate test;

extern crate core;

fn reverse( num : u128 ) -> u128 {
	let mut num = num.clone();
	let base = 10;
	let mut new_num: u128 = 0;
	for _ in 0.. {
		new_num = new_num * &base + &num % &base;
		num = &num / &base;
		if num == 0 { break };
	}
	new_num
}

fn main() {
	let mut is_lychrel = [true;10_000];

	'n: for n in 0..10_000 {
		//if n % 100 == 0 { println!("{}", n); }

		let mut num = n as _;
		num = &num + reverse(num);
		let mut reversed = reverse(num);
		let mut nr_reversals = 1;

		while &num != &reversed {
			num = num + reversed;
			reversed = reverse(num);
			nr_reversals += 1;
			if nr_reversals > 50 { continue 'n };
		}
		is_lychrel[n] = false;
	}

	let count = is_lychrel.iter().filter(|&&is_lych| is_lych).count();
	println!("{}", count);
}

#[bench]
fn bench(b: &mut test::Bencher) {
	b.iter(main)
}
