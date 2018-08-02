#![feature(test)]
#![feature(iter_arith)]
#![feature(step_by)]
extern crate test;
// map numbers 0...14 to 0...9
fn index(mut num: usize) -> usize {
    match num {
        4 | 7 | 10 | 13 => num -= 2,
        _ => (),
    };
    match num {
        0...3   => num,
        5...6   => num-1,
        8...9   => num-2,
        11...12 => num-3,
        14      => 1,
        _ => unreachable!(),
    }
}

#[inline]
fn line_valid(start: usize, gon_ring: [u8; 10], total: u8) -> bool {
    (start..start+3).map(|n| gon_ring[index(n)]).sum::<u8>() == total
}

fn generate_solutions(
    pos: usize,
    mut total: u8,
    gon_ring: [u8; 10],
    digit_taken: [bool; 10],
    solutions: &mut Vec<[u8; 15]>
) {
    match pos {
        // check or compute total at 3*n
        3 => total = gon_ring.iter().take(3).sum(),
        6 | 9 | 12 | 15 if !line_valid(pos-3, gon_ring, total) => return,
        // pre-filled, skip to next cell
        4 | 7 | 10 | 13 | 14 => {
            generate_solutions(pos+1, total, gon_ring, digit_taken, solutions);
            return;
        },
        // base case, valid solution found
        15 => {
            // find minimal external node
            let min = (3..12+1).step_by(3)
                .map(|n| gon_ring[index(n)])
                .min()
                .unwrap();
            let skip = 3 * (1 +
                (3..12+1).step_by(3)
                .map(|n| gon_ring[index(n)])
                .position(|el| el == min)
                .unwrap()
            );

            // convert array of 10 numbers to array of 15 (duplicates)
            let mut solution_set = [0; 15];
            // iterate over indices % 15
            for (i, idx) in (0..15).cycle().skip(skip).take(15).map(index).enumerate() {
                solution_set[i] = gon_ring[idx];
            }
            solutions.push(solution_set);
            return
        }
        _ => (),
    };
    // fill with every possibility, go to next cell
    for n in (1..10+1).filter(|&n| !digit_taken[n as usize - 1]) {
        let mut new_ring = gon_ring;
        let mut new_taken = digit_taken;
        new_ring[index(pos)] = n;
        new_taken[n as usize - 1] = true;
        generate_solutions(pos+1, total, new_ring, new_taken, solutions);
    }
}

fn main() {
    let mut solutions = vec![];
    let mut gon_ring = [0; 10];
    let mut digit_taken = [false; 10];
    gon_ring[0] = 10;
    digit_taken[10-1] = true;
    generate_solutions(1, 0, gon_ring, digit_taken, &mut solutions);

    let max = solutions.iter()
        .map(|arr| // convert array of digits to a number
            arr.iter()
            .fold(0, |sum, &digit| {
                match digit {
                    1...9 => sum*10 + digit as u64,
                    10    => sum*100 + digit as u64,
                    _ => unreachable!(),
                }
            }) )
        .max()
        .unwrap();

    println!("{}", max);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
