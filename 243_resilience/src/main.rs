extern crate euler_utils;
use euler_utils::prime;

fn main() {
    let totients = prime::Phi32::new(1_000_000_000);
    println!("{}",
        (1..).zip(
                totients.iter().map(|&phi| phi as usize)
            )
            .find(|&(n, phi)| phi*94744 < 15499*(n-1))
            .unwrap().0
    );
}
