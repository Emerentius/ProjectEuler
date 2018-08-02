#![feature(test)]
#![feature(step_by)]
fn main() {
    let mut L = [0u8;1_000_001];

    for mut tile_count in (8..1_000_000+1).step_by(4) { // side_length >= 3
        // for next line around square, next_side_length = old_side_length+2
        for next_ring_tile_count in (tile_count+8..1_000_000+1).step_by(8) {
            L[tile_count] += 1;
            tile_count += next_ring_tile_count;
            if tile_count > 1_000_000 { break }
        }
    }
    print!("{}", L.iter().filter(|&&cnt| cnt >= 1 && cnt <= 10).count());
}


mod benches {
    extern crate test;
    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| ::main())
    }
}
