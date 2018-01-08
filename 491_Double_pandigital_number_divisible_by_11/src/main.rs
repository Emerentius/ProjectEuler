#![feature(test)]
extern crate test;

fn gen(dig: i64, count: i64) -> Vec<Vec<i64>> {
    let mut vec = vec![];

    if dig == 9 {
        match 10 - count {
            occ @ 0...2 => return vec![vec![occ]],
            _ => return vec![],
        }
    }

    for occ in (0..2+1).take_while(|&occ| occ+count <= 10) {
        let sub_vecs = gen(dig+1, count+occ);
        for v in sub_vecs {
            let mut vec_tmp = vec![occ];
            vec_tmp.extend(v);
            vec.push( vec_tmp );
        }
    }
    vec
}

fn main() {
    const fac10 : i64 = 10*9*8*7*6*5*4*3*2;
    const fac2 : i64 = 2;

    let combinations = gen(0,0);
    let mut counter = 0;
    for comb in combinations {
        let mut n_twos = 0;
        let mut sum : i64 = 0;

        for (dig,occ) in comb.into_iter().enumerate() {
            if occ == 2 { n_twos += 1 }
            sum += 2*(occ-1)*dig as i64;
        }

        if sum % 11 == 0 {
            counter += (fac10 / fac2.pow(n_twos) ).pow(2);
        }
    }
    println!("{}", counter*9/10);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
