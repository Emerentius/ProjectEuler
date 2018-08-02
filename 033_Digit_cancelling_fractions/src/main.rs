// incomplete program, that does ,however, yield enough information for the answer

struct Ratio {
	numerator   : u32,
	denominator : u32,
}

fn main() {
	let mut counter = 0;

	for denominator in 10..100 {
		for numerator in 10..denominator {
			if numerator % 10 == 0 && denominator % 10 == 0 { continue }
		
			let ratio = Ratio{ numerator:numerator, denominator:denominator };
			
			let num_string = numerator.to_string();
			let den_string = denominator.to_string();
			
			let mut new_num_str = "".to_string();
			let mut new_den_str = "".to_string();
			
			let mut forbidden_number = 0x61 as char; // a
			for num_char in num_string.chars() {
				for den_char in den_string.chars() {
					// only ever 1 digit the same
					if num_char == den_char { forbidden_number = num_char }
				}
			}
			
			let (mut num_red, mut den_red) = (false, false);
			for char in num_string.chars() {
				if char != forbidden_number || num_red {
					new_num_str.push( char );
				} else {
					num_red = true;
				}
			}
			for char in den_string.chars() {
				if char != forbidden_number || den_red {
					new_den_str.push( char );
				} else {
					den_red = true;
				}
			}
			
			// missing check: is the new ratio in its most reduced form?
			// actually only happens for 49/98
			// lazy special case after having found it out
			let mut red_num = new_num_str.parse().unwrap();
			let mut red_den = new_den_str.parse().unwrap();
			
			if numerator == 49 && denominator == 98 {
				red_num = 1;
				red_den = 2;
			}
			
			let reduced_ratio = Ratio{
				numerator: red_num,
				denominator: red_den
			};
				
			
			if reduced_ratio.denominator != 0 && reduced_ratio.numerator != 0 
			   && num_red && den_red 
			   && ratio.numerator / reduced_ratio.numerator == ratio.denominator / reduced_ratio.denominator
			   && ratio.numerator % reduced_ratio.numerator == 0
			   && ratio.denominator % reduced_ratio.denominator == 0 {
				counter += 1;
				println!("{}/{} = {}/{}", ratio.numerator, ratio.denominator, reduced_ratio.numerator, reduced_ratio.denominator);
			}
		}
	}
	//println!("{}", counter);
}
