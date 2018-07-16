// detect cycles in the sequence of rests
// reuse the cache
fn cycle_len(divisor: u32, rests: &mut Vec<u32>) -> usize {
	rests.clear();
	for rest in rests_iter(divisor) {
		if let Some(pos) = rests.iter().position(|&prev_rest| prev_rest == rest) {
			return rests.len() + 1 - pos;
		}
		rests.push(rest);
	}
	0
}

// create iterator over the rests of the divisions for each digit
fn rests_iter(divisor: u32) -> impl Iterator<Item=u32> {
	std::iter::repeat(()).scan(1, move |rest, ()| {
		let old_rest = *rest;
		*rest = (*rest * BASE) % divisor;
		Some(old_rest)
	}).take_while(|&rest| rest != 0)
}

const BASE: u32 = 10;

fn main() {
	let mut cache = vec![];
	let max_cycle = (1..1000)
		.map(|div| cycle_len(div, &mut cache))
		.max().unwrap();
	println!("{}", max_cycle);
}
