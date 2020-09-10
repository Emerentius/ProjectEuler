#![feature(test)]
extern crate test;
extern crate primal;

#[derive(Copy, Debug, Clone)]
struct SegmentMask(u8);
type Masks = [SegmentMask; 8];

impl SegmentMask {
    fn unlit() -> SegmentMask {
        SegmentMask(0)
    }

    fn new(num: u8) -> SegmentMask {
        SegmentMask( match num {
            0 => 0b01111110,
            1 => 0b00110000,
            2 => 0b01101101,
            3 => 0b01111001,
            4 => 0b00110011,
            5 => 0b01011011,
            6 => 0b01011111,
            7 => 0b01110010,
            8 => 0b01111111,
            9 => 0b01111011,
            _ => unreachable!(),
        })
    }

    fn n_segments_overlap(self, other: SegmentMask) -> u32 {
        (self.0 & other.0).count_ones()
    }
}

fn digits(mut num: u64) -> impl Iterator<Item = u8> {
    std::iter::from_fn(move || {
        if num == 0 { return None }
        let digit = (num % 10) as u8;
        num /= 10;
        Some(digit)
    })
}

// compute array of segment masks for a given number
fn masks(num: u64) -> Masks {
    let mut masks = [SegmentMask::unlit(); 8];
    for (mask, digit) in masks.iter_mut().zip(digits(num)) {
        *mask = SegmentMask::new(digit);
    }
    masks
}

fn count_transition_differences(old_masks: Masks, new_masks: Masks) -> u64 {
    old_masks.iter()
        .zip(new_masks.iter())
        .map(|(&old_mask, &new_mask)| 2*old_mask.n_segments_overlap(new_mask) as u64)
        .sum()
}

fn main() {
    let mut sum = 0u64;
    for mut num in primal::Primes::all()
        .skip_while(|&pr| pr < 10_000_000)
        .take_while(|&pr| pr < 20_000_000)
        .map(|pr| pr as u64)
    {
        let mut last_masks = masks(num);
        while num >= 10 {
            let new_num = digits(num).map(|digit| digit as u64).sum();
            let new_masks = masks(new_num);
            sum += count_transition_differences(last_masks, new_masks);
            last_masks = new_masks;
            num = new_num;
        }
    }
    println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
