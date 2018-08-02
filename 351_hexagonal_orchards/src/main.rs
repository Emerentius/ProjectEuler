extern crate prime; // my prime library, contains a totient sieve

const N: u32 = 100_000_000;

fn main() {
    let phis = prime::Phi32::new(N);
    let solution = (1..N+1).zip(phis.iter())
        .skip(1) // first one's visible
        .map(|(n, phi)| (n - phi) as usize)
        .fold(0, std::ops::Add::add) * 6;
    println!("{}", solution);
}
