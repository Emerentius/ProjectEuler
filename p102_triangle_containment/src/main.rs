#![feature(test)]
extern crate test;

fn contains_origin(a: (i32, i32), b: (i32, i32), c: (i32, i32) ) -> bool {
    let _2area = a.0 * (b.1 - c.1) + b.0 * (c.1 - a.1) + c.0 * (a.1 - b.1);
    let sign = _2area.signum();

    let s = (c.0 * a.1 - a.0 * c.1) * sign;
    let t = (a.0 * b.1 - b.0 * a.1) * sign;

    s > 0 && t > 0 &&
    _2area.abs() > s+t
}

fn main() {
    let triangles_string = include_str!("p102_triangles.txt");

    // parse, check, count
    let solution = triangles_string.lines()
        .map(|line| {
            // parse
            let mut coords = line.split(',')
                .map(|num_str| num_str.parse().unwrap() );
            // check
            contains_origin(
                (coords.next().unwrap(), coords.next().unwrap()),
                (coords.next().unwrap(), coords.next().unwrap()),
                (coords.next().unwrap(), coords.next().unwrap()),
            )
        })
        .filter(|&el| el)
        .count(); // count

    println!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main());
}
