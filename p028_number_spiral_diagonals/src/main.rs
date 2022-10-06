fn loop_sum (iteration: u64) -> ( u64 ) {
	let n = 2*iteration+1;
	4*n*n -6*n + 6
}
fn main() {
	let mut sum = 1; // center 1
	for i in 1..(1001-1)/2+1 { sum += loop_sum(i) };
	println!("{}", sum);
}
