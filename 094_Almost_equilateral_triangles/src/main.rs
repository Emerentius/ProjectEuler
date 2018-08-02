#![feature(test)]
#![feature(step_by)]
extern crate test;

fn main() {
    let mut last_square_at = 0;
    let mut perimeter_sum = 0;
    for side_len in (3..1_000_000_000u64 / 3).step_by(2) {
        for &diff_side in [side_len-1, side_len+1].iter() {
            // if height_sq is perfect square, area is integral
            let height_sq = side_len*side_len - diff_side*diff_side/4;

            let mut sq = last_square_at * last_square_at;
            for n in last_square_at.. {
                sq += 2*n+1;
                if sq > height_sq { break };
                if sq == height_sq { perimeter_sum += 2*side_len + diff_side }
                last_square_at = n;
            }
        }
    }
    println!("{}", perimeter_sum);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
