fn pow_mod(mut number:u64, mut power:u64, modu:u64) -> u64 {
	let mut result = 1;
	while power != 0 {
		if power % 2 != 0 { result = (result*number) % modu }
		number = (number*number) % modu;
		power /= 2;
	}
	result
}

fn main() {
    // power < 2^32
    let modu = 1_000_000_000;
    let last_ten_digits = (28_433 * pow_mod(2,7_830_457, modu)) % modu + 1;
    if last_ten_digits == modu - 1 { println!("{}", 0)}
    else { println!("{}", last_ten_digits) };
}
