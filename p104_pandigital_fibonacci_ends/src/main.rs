#![allow(non_snake_case)]
#![feature(test)]
extern crate test;

fn is_pandigital (mut num: u64) -> bool {
    // 1 through 9 pandigital, assuming num is 9 digit number
    let mut digits = [false;9];
    while num != 0 {
        if num % 10 == 0 { return false }
        digits[ (num % 10) as usize - 1] = true;
        num /= 10;
    }
    digits == [true;9]
}

fn main() {
    let (mut fir_last, mut las_last) = (1,1);
    let (mut fir_next, mut las_next) = (1,1);
    for k in 3u64.. {
        let last : u64 = las_last + las_next;
        let first : u64= fir_last + fir_next;

        las_last = las_next;
        las_next = last % 1_000_000_000;

        if first>=1_000_000_000*1_000_000_000 { // additional 9 digits
            fir_last = fir_next/10;
            fir_next = first/10;
        } else {
            fir_last = fir_next;
            fir_next = first;
        };

        if is_pandigital(fir_next/1_000_000_000) && is_pandigital(las_next) {
            println!("{}", k);
            break
        }
    };
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
