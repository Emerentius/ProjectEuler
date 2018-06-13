#![feature(test)]
#![feature(iterator_step_by)]
extern crate test;
extern crate num;
use num::integer::Integer;

fn main() {
    let limit = 1_500_000;
    let mut n_triangles : Vec<u8> = vec![0;limit+1];
    for m in 2.. {
        if 2*m*(m+1) > limit { break } // perimeter > limit, always
        for n in (1+ m%2 .. m).step_by(2) {
            let a = m*m - n*n;
            let b = 2*m*n;
            if a.gcd(&b) == 1 {
                let c = m*m + n*n;
                let perimeter = a+b+c;

                let mut perimeter_tmp = perimeter;
                while perimeter_tmp <= limit {
                    n_triangles[perimeter_tmp] += 1;
                    perimeter_tmp += perimeter;
                }
            }
        }
    }
    let count = n_triangles.into_iter().filter(|&n| n == 1).count();
    println!("{}", count);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
