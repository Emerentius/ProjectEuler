fn collatz_steps (input : i64, steps : i64) -> (i64,i64) {
	match input {
		1 => return (input, steps),
		_ if input % 2 == 0 => collatz_steps(input/2, steps+1),
		_ => collatz_steps(3*input+1, steps+1),
	}
}

fn main() {
	let mut max_collatz_steps = 0;
	let mut longest_chain_startpoint = 0;
	// RangeStep unstable language feature
	// iterate over all uneven numbers
	for n in std::iter::range_step(1,1000000, 2) {
		let steps = collatz_steps(n, 1).1; // output (1, steps)
		if steps > max_collatz_steps {
			max_collatz_steps = steps;
			longest_chain_startpoint = n;
		};
	}
	println!("Highest number of collatz steps: {} for {}", max_collatz_steps, longest_chain_startpoint);
}
