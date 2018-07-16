fn main() {
	let mut rests = vec::<u32>![];
	let mut max_cycle = 0;
	let mut temp = 0;
	let base = 10;
	for i in 1..1000 {
		let mut cycles = 0;
		temp = 1;
		rests.push( temp );
		'outer: while temp % i != 0 {
			rests.push( temp*base );
			temp = (temp*base) % i;
			for (j, &rest_tb) in rests.iter().enumerate() {
				if temp*base == rest_tb {
					cycles = rests.len() - j;
					if cycles > max_cycle { max_cycle = cycles };
					break 'outer;
				}
			}
		}
		//println!("i = {}, {} long cycle\n {:?}\n", i, cycles, rests);
		rests.clear();
	}
	println!("{}", max_cycle);
}
