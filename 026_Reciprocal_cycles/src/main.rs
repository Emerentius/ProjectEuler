fn main() {
	let mut rests = vec![];
	let mut max_cycle = 0;
	let mut temp;
	let base = 10;
	for i in 1..1000 {
		let mut cycles;
		temp = 1;
		rests.push( temp );
		'outer: while temp % i != 0 {
			rests.push( temp*base );
			temp = (temp*base) % i;
			for (j, &rest_tb) in rests.iter().enumerate() {
				if temp*base == rest_tb {
					cycles = rests.len() - j + 1;
					if cycles > max_cycle { max_cycle = cycles };
					break 'outer;
				}
			}
		}
		rests.clear();
	}
	println!("{}", max_cycle);
}
