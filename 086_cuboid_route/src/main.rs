extern crate int_sqrt;
use int_sqrt::IntSqrt;

fn main() {
    let mut counter = 0;
    for sx in 1u32.. {
        for sy in 1..sx+1 {
            for sz in 1..sy+1 {
                let min_path_sq = sx*sx + (sy+sz).pow(2);
                if min_path_sq.is_square() {
                    counter += 1;
                }
            }
        }
        if counter > 1_000_000 {
            println!("{}", sx);
            break
        }
    }
}
