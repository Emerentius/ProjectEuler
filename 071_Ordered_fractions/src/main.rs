#![feature(test)]
extern crate test;

fn main() {
    let fixed_frac = (3,7);
    let mut neighbour_frac = (2,5);
    let mut next_neighbour = neighbour_frac;
    while next_neighbour.1 <= 1_000_000 {
        neighbour_frac = next_neighbour;
        next_neighbour = ( fixed_frac.0 + neighbour_frac.0, fixed_frac.1 + neighbour_frac.1 );
    }
    println!("{}", neighbour_frac.0);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
