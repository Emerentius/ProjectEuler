#![feature(test)]
extern crate test;
fn digit_sum (mut num: u64) -> u64 {
    let mut digit_sum = 0;
    while num != 0 {
        digit_sum += num % 10;
        num /= 10;
    }
    digit_sum
}

fn main() {
    let mut sequence = vec![];
    // 162 is highest digit_sum that fits a 64 bit number
    for dig_sum in 2..162u64 + 1 {
        let mut num = dig_sum;
        // calculate all integer powers of dig_sum 2 and higher
        // less than 2^64 (stop on overflow)
        while let Some(new_num) = num.checked_mul(dig_sum) {
            num = new_num;
            if digit_sum(num) == dig_sum {
                sequence.push(num);
            }
        }
    }
    sequence.sort();
    println!("{}", sequence[29]);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
