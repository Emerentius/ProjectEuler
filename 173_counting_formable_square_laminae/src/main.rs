#![feature(test)]
#![feature(step_by)]
fn main() {
    let mut laminae_count = 0;
    for mut tile_count in (8..1_000_000+1).step_by(4) { // side_length >= 3
        // for next line around square, next_side_length = old_side_length+2
        laminae_count += 1;
        for next_ring_tile_count in (tile_count+8..1_000_000+1).step_by(8) {
            tile_count += next_ring_tile_count;
            if tile_count > 1_000_000 { break }
            laminae_count += 1;
        }
    }
    print!("{}", laminae_count);
}


mod benches {
    extern crate test;
    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| ::main())
    }
}
