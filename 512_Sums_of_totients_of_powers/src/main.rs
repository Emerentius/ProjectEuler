#![feature(test)]
#![feature(step_by)]
#![feature(iter_arith)]
extern crate test;
extern crate prime;

fn pow_mod(base: u32, mut exp: u32, modu: u32) -> u32 {
    let modu = modu as u64;
    let mut base = base as u64 % modu;
    let mut result = 1u64;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result*base) % modu;
        }
        exp /= 2;
        base = (base*base) % modu;
    }
    result as u32
}

fn f(n: u32, totients: &prime::Phi32) -> u32 {
    if n % 2 == 0 { return 0 }
    if totients[n] == n-1 { return n-1 }
    let mut sum = 1;
    for _ in 0..n-1 {
        sum = sum*n % (n+1) + 1
    }
    (sum * totients[n]) % (n+1)

    /*(
        (0..n)
            .map(|exp| pow_mod(n, exp, n+1))
            .fold(0, |sum, num| sum+num) * totients[n]
    ) % (n+1)*/

    /*
    if totients[n] == 1 { return n }
    //let sum = (n as u64).pow(n+1)-1/(n as u64 -1); // without phi
    let sum = (pow_mod(n, n+1, n+1)-1)/(n-1); // without phi
    ( (sum*totients[n]) % (n+1) )
    */
}
const N: u32 = 500_000_000;

fn main() {
    let totients = prime::Phi32::new(N);
    let g: u64 = (1..N+1)
        .step_by(2)
        .map(|i| totients[i] as u64)
        .sum();
    println!("{}", g);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
