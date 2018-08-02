#![feature(test)]
extern crate test;

fn count_rectangles(width: i64, height: i64) -> i64 {
    (width * (width+1))*(height*(height+1))/4
}

fn main() {
    let mut smallest_offset = 2_000_000;
    let mut closest_area = 0;
    let mut width = 1;
    // start at lowest height with >2_000_000 rectangles inside
    let mut height = (1..).find(|&h| count_rectangles(width, h) > 2_000_000)
        .unwrap();

    // increase width or decrease height, depending on the rectangle count
    while height > width {
        let n_rect = count_rectangles(width, height);

        let offset = (2_000_000 - n_rect).abs();
        if offset < smallest_offset {
            smallest_offset = offset;
            closest_area = width*height;
        }

        match n_rect > 2_000_000 {
            true => height -= 1,
            false => width += 1,
        }
    }
    print!("{}", closest_area);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
