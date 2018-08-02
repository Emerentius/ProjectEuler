#![feature(test)]
const MAX_EXP: usize = 200;

fn walk_tree(prev_exps: &mut Vec<usize>, m: &mut [usize; MAX_EXP]) {
    let last_exp = *prev_exps.last().unwrap();
    let n_mul = prev_exps.len() - 1;
    if last_exp > MAX_EXP || n_mul > m[last_exp - 1] {
        return
    }
    m[last_exp-1] = n_mul;
    for i in (0..prev_exps.len()).rev() {
        let new_exp = last_exp + prev_exps[i];
        prev_exps.push(new_exp);
        walk_tree(prev_exps, m);
        prev_exps.pop();
    }
}

fn main() {
    let mut m = [!0; MAX_EXP];
    walk_tree(&mut vec![1], &mut m);
    let solution: usize = m.iter().sum();
    println!("{}", solution);
}


mod bench {
    extern crate test;
    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| ::main())
    }
}
