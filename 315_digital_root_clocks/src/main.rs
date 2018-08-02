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

    fn segments_lit(self) -> u32 { self.0.count_ones() }
    fn n_segments_overlap(self, other: SegmentMask) -> u32 {
        (self.0 & other.0).count_ones()
    }
}

// compute array of segment masks for a given number
fn masks(mut num: u64) -> Masks {
    let mut masks = [SegmentMask::unlit(); 8];
    let mut position = 0;
    while num != 0 {
        masks[position] = SegmentMask::new((num % 10) as u8);
        num /= 10;
        position += 1;
    }
    masks
}

fn count_transition_differences(old_masks: Masks, new_masks: Masks) -> u64 {
    old_masks.iter()
        .zip(new_masks.iter())
        .fold(0, |sum, (&old_mask, &new_mask)| sum + 2*old_mask.n_segments_overlap(new_mask) as u64)
}

// compute digit sum for nums > 10, None used as iteration stop flag
fn digit_sum(mut num: u64) -> Option<u64> {
    if num < 10 { return None }
    let mut sum = 0;
    while num != 0 {
        sum += num % 10;
        num /= 10;
    }
    Some(sum)
}

fn main() {
    let mut sum = 0u64;
    for mut num in primal::Primes::all()
        .skip_while(|&pr| pr < 10_000_000)
        .take_while(|&pr| pr < 20_000_000)
        .map(|pr| pr as u64)
    {
        let mut last_masks = masks(num);
        while let Some(new_num) = digit_sum(num) {
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
