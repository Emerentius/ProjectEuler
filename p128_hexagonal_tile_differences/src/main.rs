#![feature(test)]
extern crate test;

extern crate primal;
//use primal::is_prime; // deterministic Miller-Rabin

fn first_num(n_ring: u64) -> u64 { n_ring*(n_ring-1)*3 + 2 }
fn ring_length(n_ring: u64) -> u64 { n_ring * 6 }
fn last_num(n_ring: u64) -> u64 { first_num(n_ring) + ring_length(n_ring) - 1 }

fn main() {
    let primes = primal::Sieve::new(1_000_000);
    let is_prime = |num: u64| primes.is_prime(num as usize);

    // create iterator over all n where PD(n) == 3
    // for every ring, look at first and last number
    // impossible for all others
    let mut pd3_tile_iter = (1..).map(|n_ring| {
        let diff_first_last = ring_length(n_ring) - 1;
        if !is_prime(diff_first_last) { return (None, None) }

        let diff_first_top_left = ring_length(n_ring) + 1;
        let diff_first_top_right = diff_first_top_left + ring_length(n_ring+1) - 2;
        let first = if is_prime(diff_first_top_left) && is_prime(diff_first_top_right) {
            Some(first_num(n_ring)) // PD(first) == 3
        } else { None };

        let diff_last_top_right = ring_length(n_ring+1) - 1;
        let diff_last_bot_left = ring_length(n_ring) - 1 + ring_length(n_ring-1);
        let last = if is_prime(diff_last_top_right) && is_prime(diff_last_bot_left) {
            Some(last_num(n_ring)) // PD(last) == 3
        } else { None };

        (first, last)
    }).flat_map(|(first, last)| first.into_iter().chain(last));

    // 0 indexed, 1999 is 2000th element
    let solution = pd3_tile_iter.nth(1999).unwrap();
    println!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
