// I'm not responsible for premature loss of sight if you wish to inflict this code upon yourself
extern crate test;
use std::iter::range_step;

fn sum(p1:u32,p2:u32, p5:u32, p10:u32, p20:u32,p50:u32, p100:u32) -> u32 { p1+p2+p5+p10+p20+p50+p100 }

fn main() {
	let mut variations = 1; // 2 pound coin loop left out
	
	let (p1,p2,p5,p10,p20,p50,p100) = (0,0,0,0,0,0,0);
	for p100 in range_step(0,201-sum(p1,p2,p5,p10,p20,p50,p100), 100) {
		for p50 in range_step(0, 201-sum(p1,p2,p5,p10,p20,p50,p100), 50) {
			for p20 in range_step(0, 201-sum(p1,p2,p5,p10,p20,p50,p100), 20) {
				for p10 in range_step(0,201-sum(p1,p2,p5,p10,p20,p50,p100), 10) {
					for p5 in range_step(0, 201-sum(p1,p2,p5,p10,p20,p50,p100), 5) {
						for p2 in range_step(0,201-sum(p1,p2,p5,p10,p20,p50,p100),2) {
							for p1 in range_step(0,201-sum(p1,p2,p5,p10,p20,p50,p100),1) {
								let sum = sum(p1,p2,p5,p10,p20,p50,p100);
								if sum == 200 { variations += 1 };
							}
						}
					}
				}
			}
		}
	}
	
	println!("{}", variations)
}

#[bench]
fn bench(b: &mut test::Bencher) { b.iter(|| main() ) }