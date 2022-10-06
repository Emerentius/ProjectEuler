#![feature(test)]
extern crate test;

const LIMIT: usize = 50_000_000;

fn main() {
    let mut nums = vec![0; LIMIT];

    for a in 1..LIMIT+1 {
        let mut b = 4 - a % 4;
        while a*b < LIMIT && b < 3*a {
            nums[a*b-1] += 1;
            b += 4;
        }
    }

    println!("{}",
        nums.into_iter().filter(|&count| count == 1).count()
    );
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
