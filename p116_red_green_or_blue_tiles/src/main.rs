#![feature(test)]
extern crate test;

/*
fn count_tilings(n_tiles: i64, block_len: i64) -> i64 {
    let mut sum = 0;
    for pos in (0..n_tiles - block_len + 1) {
        sum += 1 + count_tilings(n_tiles-pos-block_len, block_len);
    }
    sum
}
*/

const N: usize = 50;
const N_PLUS: usize = 51;

fn main() {
    let mut n_tilings_red = [0; N_PLUS];
    let mut n_tilings_green = [0; N_PLUS];
    let mut n_tilings_blue = [0; N_PLUS];

    n_tilings_red[2] = 1u64;
    n_tilings_green[3] = 1u64;
    n_tilings_blue[4] = 1u64;

    for n_tiles in 3..N_PLUS {
        for pos in 0..n_tiles - 2 + 1 {
            n_tilings_red[n_tiles] += 1 + n_tilings_red[n_tiles - pos - 2]
        }
    }
    for n_tiles in 4..N_PLUS {
        for pos in 0..n_tiles - 3 + 1 {
            n_tilings_green[n_tiles] += 1 + n_tilings_green[n_tiles - pos - 3]
        }
    }
    for n_tiles in 5..N_PLUS {
        for pos in 0..n_tiles - 4 + 1 {
            n_tilings_blue[n_tiles] += 1 + n_tilings_blue[n_tiles - pos - 4]
        }
    }

    println!("{}", n_tilings_red[N] + n_tilings_green[N] + n_tilings_blue[N]);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
