fn pow_mod(mut number: u128, mut power: u128, modu: u128) -> u128 {
	let mut result = 1;
	while power != 0 {
		if power % 2 != 0 { result = (result*number) % modu }
		number = (number*number) % modu;
		power /= 2;
	}
	result
}

fn main() {
    // power < 2^64
    let modu = 10_000_000_000;
    let last_ten_digits = ((28_433 * pow_mod(2,7_830_457, modu)) + 1) % modu;
	println!("{}", last_ten_digits);
}
