extern crate num; // 0.1.42
use num::integer::Integer;

fn p(s: u64, max: u64) -> u64 {
    // (n + k) = (n - 1 + k + 1) = (n - 1) (mod k + 1)
    // ⇒ (n - 1) % (k + 1) == 0 ∀ k in (1, N) but NOT N
    let lcm = (2..s+1).fold(1, |lcm, num| lcm.lcm(&num));
    let lcm_next = lcm.lcm(&(s+1));
    (max - 2) / lcm - (max - 2) / lcm_next
}

fn main() {
    println!("{}", (1..32)
        .map(|i| p(i, 4u64.pow(i as u32)))
        .sum::<u64>()
    );
}
