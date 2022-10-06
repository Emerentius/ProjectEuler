#![feature(test)]
extern crate test;

fn main() {
    print!("{}",
        (1..2u32.pow(30)+1).filter(|&n| n ^ 2*n ^ 3*n == 0).count()
    );
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
