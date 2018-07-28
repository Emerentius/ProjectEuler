extern crate num;
extern crate bitvec;
use num::integer::gcd;
use bitvec::BitVec;
use std::collections::HashSet;
/*
fn int_root (num: u64, inside: std::ops::Range<u64>) -> u64 {
    for n in inside {
        if n*n == num { return n }
    }
    unreachable!();
}*/

fn main() {
    let max_n = 1_000_000_000;
    let sqrt_max_n = (max_n as f64).sqrt() as usize;
    println!("{}", sqrt_max_n);
    //let limit = (max_n as f64).sqrt() as usize;
    /*for n in 1..max_n+1 {
        is_square.set((n*n) as usize, true);
    }*/

    /*let mut sum = 0;
    for r_b in 1..max_n+1 {
        for r_a in 1..r_b+1 {
            let xx_b = r_a*r_b;
            if !is_square[xx_b as usize] { continue }
            let x_b = 2*int_root(xx_b, r_a..r_b+1);
            let xx_b = xx_b*4;
            for r_c in 1..r_a {
                let xx_c = r_a*r_c;
                if !is_square[xx_c as usize] { continue }
                let x_c = 2*int_root(xx_c, r_c..r_a+1);
                let xx_c = 4*xx_c;
                if xx_b + xx_c -2*x_b*x_c == 4*r_b*r_c {
                    println!("r_a, r_b, r_c: {}, {}, {}", r_a, r_b, r_c);
                    sum += r_a+r_b+r_c;
                }
            }
        }
    }*/
    /*let mut sum = 0;
    for r_b in 1..max_n+1 {
        if !is_square[r_b as usize] { continue }
        for r_a in 1..r_b+1 {
            let xx_b = r_a*r_b;
            if !is_square[xx_b as usize]
            && !is_square[r_a as usize]
            {
                continue
            }
            let x_b = 2*int_root(xx_b, r_a..r_b+1);
            let xx_b = xx_b*4;
            for r_c in 1..r_a {
                if !is_square[r_c as usize] { continue }
                let xx_c = r_a*r_c;
                if !is_square[xx_c as usize] { continue }
                let x_c = 2*int_root(xx_c, r_c..r_a+1);
                let xx_c = 4*xx_c;
                if xx_b + xx_c -2*x_b*x_c == 4*r_b*r_c {
                    println!("r_a, r_b, r_c: {}, {}, {}", r_a, r_b, r_c);
                    sum += r_a+r_b+r_c;
                }
            }
        }
    }*/
    //let mut solutions = vec![];
    //let mut linearly_dependent = HashSet::new();
    //linearly_dependent

    /*for b in 1..sqrt_max_n+1 {
        let r_b = b*b;
        for a in (1..b+1) {
            let r_a = a*a;*/
    let mut sum = 0;
    for a in 1..sqrt_max_n+1 {
        //if a % 1000 == 0 { println!("{}", a) }
        let r_a = a*a;
        for c in 1..a {
            let r_c = c*c;
            let x_c = 2*a*c;
            //for b in a..sqrt_max_n+1 {
            for b in (a*c..sqrt_max_n+1).step_by(a*c) {
                let r_b = b*b;
                //let xx_b = 4*r_a*r_b;
                let x_b = 2*a*b;

                if x_b*x_b + x_c*x_c == 4*r_b*r_c + 2*x_b*x_c {

                    //if linearly_dependent.contains(&(b,a,c)) {
                    if gcd(a,gcd(b,c)) != 1 {
                        continue
                    }
                    //println!("r_b, r_a, r_c: {}, {},{}", r_b, r_a, r_c);
                    //println!("a,b,c: {}, {}, {}  \t\t{}, {}\ta*c/b, a-c", a,b,c, a*c/b, a-c);
                    let n_sols = max_n / r_b;
                    sum += n_sols*(n_sols+1)/2 * (r_a + r_b + r_c);
                    //solutions.push((b,a,c));
                    /*for m in (2..).take_while(|m| m*b <= sqrt_max_n) {
                        linearly_dependent.insert((m*b,m*a,m*c));
                    }*/
                }
            }
        }
    }

    println!("{}", sum);
}
