#![feature(test)]
extern crate test;

fn next(n: u64) -> (char, u64) {
    match n % 3 {
        0 => ('D', (n / 3)),
        1 => ('U', (4*n + 2)/3),
        2 => ('d', (2*n-1)/3),
        _ => unreachable!(),
    }
}

fn nth_step(n: u64, position: u64) -> char {
    (0..position)
        .fold(('0', n), |(_, n), _| next(n)).0
}

fn main() {
    let target_sequence = "UDDDUdddDDUDDddDdDddDDUDDdUUDd";

    let mut start = 10u64.pow(15);
    let mut step_size = 1;

    for (i, ch) in target_sequence.chars().enumerate() {
        start = (start..).step_by(step_size)
            .take(3)
            .find(|&n| nth_step(n, 1+i as u64) == ch).unwrap();
        step_size *= 3;
    }
    print!("{}", start);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
