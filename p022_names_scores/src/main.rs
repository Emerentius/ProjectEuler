fn name_score(name: &str) -> u64 {
	name.chars().map(|ch| ch as u64 - b'A' as u64 + 1).sum()
}

fn main () {
	let input = include_str!("input.txt");
	let mut name_list : Vec<_> = input.split_whitespace().collect();
	name_list.sort();
	let sum: u64 = (1..).zip(name_list)
		.map(|(i, name)| i*name_score(name))
		.sum();
	println!("{}", sum);
}
