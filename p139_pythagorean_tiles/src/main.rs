#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime;
use prime::FareySequence; // my prime library

const N: usize = 100_000_000;

fn main() {
    let farey_seq = FareySequence::new(70_71); // m < sqrt(N/2) ~ 7071
    let perimeter = |n,m| 2*m*(m+n);

    let mut counter = 0;
    for (n,m) in farey_seq.filter(|&(n,m)| (m-n) % 2 == 1 && perimeter(n,m) < N)
    {
        let a = m*m - n*n;
        let b = 2*m*n;
        let c = m*m + n*n;
        let (a,b) = if a > b { (b,a) } else { (a,b) };

        if c % (b-a) == 0 {
            counter += N/(a+b+c);
        }
    }
    println!("{}", counter);

    // println!("{}", farey.filter(|&(n,m)| perimeter(n,m) < N).count());
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
