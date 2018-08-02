#![feature(test)]
#![feature(step_by)]
extern crate test;

fn main() {
    let mut sum = 0;
    for a in 3..1001 {
        let mut max = 2*a;
        let aa = a*a;
        let max = (1..2*a).step_by(2)
            .map(|n| 2*n*a % aa)
            .max().unwrap();
        sum += max;
    }
    println!("{}", sum);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
