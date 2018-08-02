extern crate primal;
use primal::is_prime;

fn main() {
    let count = (1u64..50_000_000+1).filter(|n| is_prime(2*n*n-1) ).count();
    println!("{}", count);
}
