
#![feature(test)]
extern crate test;

// how many cubes in are in layer `n`
fn cubes(x: u64, y: u64, z: u64, n: u64) -> usize {
    assert!(n > 0);
    (2*(x*y + x*z + y*z) + 4*(n-1)*(z + x + y + (n-2))) as usize
}

const MAX: usize = 20_000;

fn main() {
    let mut cuboid_occurences = vec![0; MAX];
    for x in (1..).take_while(|&x| cubes(x, 1, 1, 1) < MAX) {
        for y in (x..).take_while(|&y| cubes(x, y, 1, 1) < MAX) {
            for z in (y..).take_while(|&z| cubes(x, y, z, 1) < MAX) {
                for n_cubes in (1..).map(|n| cubes(x, y, z, n))
                    .take_while(|&n_cubes| n_cubes < MAX)
                {
                    cuboid_occurences[n_cubes] += 1;
                }
            }
        }
    }

    let solution = cuboid_occurences.iter().position(|&count| count == 1000).unwrap();
    println!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(main)
}
