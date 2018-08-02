#![feature(test)]
extern crate test;

fn generate_brick_lines(mut line: Vec<u8>, space_left: u8, storage: &mut Vec<Vec<u8>>) {
    match space_left {
        0 => unreachable!(),
        1 => (), // invalid line, do nothing
        2 | 3 => {
            line.push(space_left);
            storage.push(line);
        },
        _ => {
            let mut line2 = line.clone();
            line.push(2);
            line2.push(3);
            generate_brick_lines(line, space_left-2, storage);
            generate_brick_lines(line2, space_left-3, storage);
        }
    }
}

fn count_possible_walls(poss_per_line: Vec<usize>, height_left: u8, allowed_next: &Vec<Vec<usize>>) -> usize {
    if height_left == 0 {
        return poss_per_line.into_iter().sum()
    }
    let mut new_poss_per_line = vec![0; poss_per_line.len()];
    for (line_nr, &n_poss) in poss_per_line.iter().enumerate() {
        for &poss_next_line in &allowed_next[line_nr] {
            new_poss_per_line[poss_next_line] += n_poss;
        }
    }
    count_possible_walls(new_poss_per_line, height_left-1, allowed_next)
}

const WIDTH: u8 = 32;
const HEIGHT: u8 = 10;

fn main() {
    // compute all possible brick lines
    let mut brick_lines = vec![];
    generate_brick_lines(vec![], WIDTH, &mut brick_lines);

    // compute crack masks for each brick line
    let mut cracks = vec![];
    for brick_line in &brick_lines {
        let mut crack = 0;
        let mut line_cracks = 0u32; // bitmask
        // leave out edge cracks
        for &num in &brick_line[..brick_line.len()-1] {
            crack += num;
            line_cracks |= 1 << crack;
        }
        cracks.push(line_cracks);
    }

    // compute what brick lines may follow one another (i.e. don't have running cracks)
    let allowed_next_lines = cracks.iter()
        .map(|&cracks1| { // some duplicated work here, for-loop + looking back probably more efficient
            cracks.iter()
                .enumerate()
                .filter(|&(_, &cracks2)| cracks1 & cracks2 == 0) // no overlapping cracks
                .map(|(j, _)| j)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    /*
    let mut poss_per_line = vec![1; brick_lines.len()];
    for _ in 0..HEIGHT-1 {
        let mut new_poss_per_line = vec![0; poss_per_line.len()];
        for (line_nr, &n_poss) in poss_per_line.iter().enumerate() {
            for &poss_next_line in &allowed_next_lines[line_nr] {
                new_poss_per_line[poss_next_line] += n_poss;
            }
        }
        poss_per_line = new_poss_per_line;
    }
    */
    //println!("{}", poss_per_line.into_iter().sum::<u64>());
    println!("{}", count_possible_walls(vec![1; brick_lines.len()], HEIGHT-1, &allowed_next_lines));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
