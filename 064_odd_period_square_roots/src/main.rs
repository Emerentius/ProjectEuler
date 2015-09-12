#![feature(test)]
extern crate test;
use std::collections::BTreeSet;

// Periodic continued fractions only
struct ContFract {
    m:   u16,
    d:   u16,
    a_0: u16,
    a:   u16,
    num: u16,
    fin: bool,
}

impl ContFract {
    fn new(num: u16) -> ContFract {
        let a_0 = (num as f32).sqrt() as u16;
        ContFract {
            m: 0,
            d: 1,
            a_0: a_0,
            a: a_0,
            num: num,
            fin: false,
        }
    }
}

impl Iterator for ContFract {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fin {
            return None
        }
        if self.a == 2*self.a_0 {
            self.fin = true
        }

        let old = self.a;
        self.m = self.d * self.a - self.m;
        self.d = (self.num - self.m * self.m)/self.d;
        self.a = (self.a_0 + self.m) / self.d;

        Some(old)
    }
}

fn main() {
    let squares: BTreeSet<u16> = (1..100+1).map(|x| x*x).collect();

    println!("{}",
        (1..10_000+1).filter(|n| !squares.contains(n))
            .filter(|&n| (ContFract::new(n).count() - 1) % 2 == 1)
            .count()
    )
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
