#![feature(test)]
#![feature(iterator_step_by)]
extern crate test;
extern crate vec_map;
use vec_map::VecMap;

fn main() {
    let divisor_sums = {
        let mut divisor_sums = vec![1; 1_000_000+1]; // 1 accounted for
        for div in 2..500_000+1 {
            for num in (2*div..1_000_000).step_by(div) {
                divisor_sums[num] += div;
            }
        }
        divisor_sums
    };

    let mut is_chain = VecMap::with_capacity(1_000_001);
    let mut chains = vec![];
    let mut cache = vec![];

    for n in 1..1_000_000+1 {
        let mut div_sum = divisor_sums[n];
        cache.push(div_sum);
        loop {
            // known to be no chain or complete chain
            if is_chain.get(&div_sum).is_some() {
                is_chain.extend(
                    cache.drain(..).map(|num| (num, None))
                );
                break;
            }
            if div_sum > 1_000_000 {
                for num in cache.drain(..) {
                    is_chain.insert(num, None);
                }
                break
            }

            div_sum = divisor_sums[div_sum];
            // chain detected
            if let Some(pos) = cache.iter().position(|&x| x == div_sum) {
                let mut chain = vec![];
                for &num in &cache[pos..] {
                    chain.push(num);
                    is_chain.insert(num, Some(chains.len()));
                }
                for &num in &cache[..pos] {
                    is_chain.insert(num, None);
                }
                chains.push(chain);
                cache.clear();
                break;
            }
            cache.push(div_sum);
        }
    }
    let max_chain_len = chains.iter().map(|ch| ch.len()).max().unwrap();
    let longest_chain_index = chains.iter()
        .position(|ch| ch.len() == max_chain_len)
        .unwrap();
    println!("{}", chains[longest_chain_index].iter().min().unwrap());
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
