#![feature(test)]
extern crate test;

fn pow_mod(mut number:u64, mut power:u32, modu:u64) -> u64 {
	let mut result = 1;
	while power != 0 {
		if power % 2 != 0 { result = (result*number) % modu }
		number = (number*number) % modu;
		power /= 2;
	}
	result
}

fn hyper_exp(num: u64, n_exponents: u64, modu: u64) -> u64 {
    match n_exponents {
        1 => num,
        _ => pow_mod(num, hyper_exp(num, n_exponents-1, modu) as u32, modu)
    }
}

fn main() {
    println!("{}", hyper_exp(1777,1855,10u64.pow(8)))
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
