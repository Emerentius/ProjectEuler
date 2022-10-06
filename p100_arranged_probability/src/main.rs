#![feature(test)]
extern crate test;

fn main() {
    let mut p = 15;
    let mut q = 21;

    while q < 1_000_000_000_000u64 {
        let p_next = 3*p + 2*q - 2; // http://www.alpertron.com.ar/QUAD.HTM
        let q_next = 4*p + 3*q -3;  // Why the fuck do these work? They are for the substited variables, not p and q directly

        p = p_next;
        q = q_next;
    }
    print!("{}", p);
}

#[bench]
fn bench(b: &mut test::Bencher) { b.iter(|| main()) }
