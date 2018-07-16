// rests of 1/divisor
fn cycle_len(divisor: u32, rests: &mut Vec<u32>) -> usize {
	rests.clear();
	let mut rest = 1;
	while rest % divisor != 0 {
		rests.push(rest);
		rest = (rest*BASE) % divisor;
		if let Some(pos) = rests.iter().position(|&prev_rest| prev_rest == rest) {
			return rests.len() + 1 - pos;
		}
	}
	0
}

const BASE: u32 = 10;

fn main() {
	let mut cache = vec![];
	let max_cycle = (1..1000)
		.map(|div| cycle_len(div, &mut cache))
		.max().unwrap();
	println!("{}", max_cycle);
}
