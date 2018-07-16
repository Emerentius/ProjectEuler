#![feature(test)]
extern crate test;

fn main() {
	let limit = 1000;
	for c in 1..limit+1 {
		for b in 1..limit+1-c {
			let a : isize = limit - c - b;
			if b >= c { break; }
			if a < b && a*a + b*b == c*c {
				println!("a*b*c = {}*{}*{} = {}",a,b,c,a*b*c);
				break;
			}
		};
	}
}

#[bench]
fn bench(b:&mut test::Bencher) {
	b.iter(|| main() )
}
