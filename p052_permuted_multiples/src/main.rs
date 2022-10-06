#![feature(test)]
extern crate test;

fn next_order_of_magnitude ( num:u64 ) -> u64 {
	let mut next = 10;
	while next < num { next *= 10 }
	next
}

fn main() {
	let mut num = 9;
	'num:loop { //'
		num += 9;
		let mut bytes1 = num.to_string().into_bytes();
		bytes1.sort();
			for mult in (2..7).rev() {
				let mut bytes2 = (num*mult).to_string().into_bytes();
				if mult == 6 && bytes2.len() != bytes1.len() {
					// if num*6 is one order of magnitude higher
					// fast forward one order of magnitude, keep divisibility by 9
					num = next_order_of_magnitude(num) + 8;
					continue 'num;
				}

				bytes2.sort();
				if bytes1 != bytes2 {
					continue 'num;
				}
			}
			println!("{}", num);
			break;
	}
}

#[bench]
fn bench ( b: &mut test::Bencher) {
	b.iter( || main() );
}
