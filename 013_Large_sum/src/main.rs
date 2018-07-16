fn max (a:i32, b:i32) -> i32 { if a>b { a } else { b } }

fn main() {
	let input = ;
	
	// read into array
	let len = 50;
	let amount = 100;
	let mut array : [i32;100] = [[0;20];20];
	for column in 0..len_height {
		for line in 0..len_height {
			let pos = 3*column+3*20*line;
			let slice = &input[pos..pos+2];
			array[column][line] = slice.parse().ok().expect("Parse error");
		}
	}
	
	let mut max_product = 0;
	for column in 0..len_height {
		for line in 0..len_height {
			max_product = max( max_line_product(&array, column, line) , max_product);
		}
	}
	println!("{}", max_product);
}
