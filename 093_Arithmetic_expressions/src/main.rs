#![feature(collections)]
#![feature(test)]
extern crate test;

fn helper_possible_results(prev_results: Vec<i32>, num:i32, residual_ratios: Vec<[i32; 2]>) -> (Vec<i32>, Vec<[i32; 2]>) {
    let mut new_results = vec![];
    let mut new_residual_ratios = vec![];
    for res in prev_results {
        new_results.push(res + num);
        new_results.push(res * num);
        new_results.push(-res * num);
        new_results.push(res - num);
        new_results.push(num - res);
        if res % num == 0 {
            new_results.push(res/num);
            new_results.push(-res/num);
        }
        else {
            new_residual_ratios.push([res,num]);
            new_residual_ratios.push([-res, num]);
        }
    }
    for [numer, denom] in residual_ratios {
        new_residual_ratios.push([numer, denom*num]);
        new_residual_ratios.push([numer - denom*num, denom]);
        new_residual_ratios.push([numer + denom*num, denom]);
        if num*numer % denom == 0 { new_results.push(num*numer / denom) }
        else { new_residual_ratios.push([num*numer, denom]) }
    }

    new_results.sort();
    new_results.dedup();
    (new_results, new_residual_ratios)
}

fn calc_possible_results (a:i32, b:i32, c:i32, d:i32) -> Vec<i32> {
    let mut overall_results = vec![];
    for perm in [a,b,c,d].permutations() {
        if let [a,b,c,d] = &perm[..] {
            let (result1, residual1) = helper_possible_results(vec![a], b, vec![]);
            let (result2, residual2) = helper_possible_results(result1, c, residual1);
            let (result3, _) = helper_possible_results(result2, d, residual2);
            overall_results.push_all( &result3[..] );
            overall_results.sort();
            overall_results.dedup();
        }
    }
    overall_results
}

fn calc_chain_length(possible_results:Vec<i32>) -> u32 {
    possible_results.into_iter()
        .skip_while(|&num| num <= 0 )
        .enumerate()
        .take_while(|&(i, num)| (i+1) as i32 == num)
        .count() as u32
}

fn main() {
    let mut max_chain_length = 0;
    let mut tuple_longest_chain = (0,0,0,0);
    for a in 1..9+1-3 {
        for b in a+1..9+1-2 {
            for c in b+1..9+1-1 {
                for d in c+1..9+1 {
                    let possible_nums = calc_possible_results(a,b,c,d);
                    let chain_length = calc_chain_length(possible_nums);
                    if chain_length > max_chain_length {
                        max_chain_length = chain_length;
                        tuple_longest_chain = (a,b,c,d);
                    }

                }
            }
        }
    }
    let r = tuple_longest_chain;
    println!("length of longest chain: {}", max_chain_length);
    println!("{}{}{}{}", r.0,r.1,r.2,r.3)
}

#[bench]
fn bench(b:&mut test::Bencher) {
    b.iter(|| main())
}
