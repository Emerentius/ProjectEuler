fn main() {
	let mut max_solutions = 0;
	let mut p_max_solutions = 0;
	'per:for perimeter in 1..1001 {
		let mut solutions = 0;
		'a:for a in 1..perimeter/2+1 {
			'b:for b in a..perimeter/2+1 {
				let c = perimeter - a - b;
				if c <= 0 { continue 'a }
				if a*a+b*b == c*c { solutions += 1 }
			}
		}
		if solutions > max_solutions {
			max_solutions = solutions;
			p_max_solutions = perimeter }
	}
	println!("{}", p_max_solutions);
}
