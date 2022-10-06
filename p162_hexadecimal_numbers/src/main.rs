#![feature(test)]
extern crate test;

const N: usize = 16;

fn main() {
    // possible numbers
    // count[length][bitmask for (starts_with_0, contains 01A (3 bits))]
    let mut count = [[0u64; 16]; N];
    count[0][0b0000] = 13;
    count[0][0b0001] = 1;
    count[0][0b0010] = 1;
    count[0][0b1100] = 1;

    for len in 1..N {
        let (last_count, tail) = (&mut count[len-1..]).split_first_mut().unwrap();
        let next_count = tail.first_mut().unwrap();
        // Iterate over bitfields of (0, 1, A, anything else)
        for &(bitfield, occ) in &[(0b0000, 13), (0b0001, 1), (0b0010, 1), (0b1100, 1)] {
            for (old_bitfield, &counts) in last_count.iter().enumerate() {
                // delete starts_with_zero_flag before combining fields
                // is added back in for 0
                next_count[(old_bitfield & 0b0111) | bitfield] += counts * occ;
            }
        }
    }
    println!("{:X}", count.iter().map(|counts| counts[0b0111]).sum::<u64>());
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main());
}
