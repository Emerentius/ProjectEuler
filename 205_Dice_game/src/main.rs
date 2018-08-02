#![feature(test)]
extern crate test;

fn checked_next_throw (throw: &mut[u8] , max: u8) -> bool {
    let mut carry = true;
    for face in throw.iter_mut() {
        if !carry { break }
        if *face == max {
            *face = 1;
        } else {
            *face += 1;
            carry = false;
        }
    }
    !carry
}

fn main() {
    let mut throw4 = [1;9];
    let mut throw6 = [1;6];

    let mut eyes_occ4 = [0;37];
    let mut eyes_occ6 = [0;37];

    // first throw slips through
    eyes_occ4[9] = 1;
    eyes_occ6[6] = 1;

    while checked_next_throw(&mut throw4, 4) {
        let throw = throw4.iter().fold(0, |sum, &face| sum + face);
        eyes_occ4[throw as usize] += 1;
    }

    while checked_next_throw(&mut throw6, 6) {
        let throw = throw6.iter().fold(0, |sum, &face| sum + face);
        eyes_occ6[throw as usize] += 1;
    }

    let n_throws4 = 4u64.pow(9);
    let n_throws6 = 6u64.pow(6);

    let mut wins4 : u64 = 0;
    let mut n_throws4_higher = n_throws4;

    for (&thr4, &thr6) in eyes_occ4.iter().zip(eyes_occ6.iter()) {
        n_throws4_higher -= thr4;
        wins4 += n_throws4_higher * thr6;
    }

    //println!("{}/{}", wins4, n_throws4*n_throws6);
    println!("{:.7}", wins4 as f64 / (n_throws4*n_throws6) as f64);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
