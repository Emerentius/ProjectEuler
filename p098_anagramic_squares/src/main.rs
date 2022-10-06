#![feature(test)]
extern crate test;

extern crate euler_utils;
use euler_utils::combinatorics::PartialPermutationsStreamIter;
use std::collections::{HashMap, BTreeMap};

fn count_letters(word: &str) -> [u8; 26] {
    let mut count = [0; 26];
    for ch in word.chars() { // ascii
        count[(ch as u8 - b'A') as usize] += 1;
    }
    count
}

fn main() {
    let words_string = include_str!("p098_words.txt");

    // find anagrams
    let mut anagram_map = HashMap::new();
    for word in words_string.split(',').map(|s| s.trim_matches('"')) {
        anagram_map.entry(count_letters(word))
            .or_insert(vec![])
            .push(word);
    }

    // filter out words without anagrams
    let anagrams = anagram_map.into_iter()
        .map(|(_, words)| words)
        .filter(|words| words.len() > 1)
        .collect::<Vec<_>>();

    // precompute squares
    let squares = (1..100_000).map(|n| n*n).collect::<Vec<_>>();
    let is_square = |num| {
        match num % 16 {
            0 | 1 | 4 | 8 | 9 => squares.binary_search(&num).is_ok(),
            _ => false,
        }
    };

    let digits = [0u64,1,2,3,4,5,6,7,8,9];
    // a few caches
    let mut char_to_digit = BTreeMap::new();
    let mut word_nums = vec![]; // cache

    // iterator for partial permutations (amount of elements variable)
    // for bruteforcing all possible char to digit mappings
    let mut permutations_iter = PartialPermutationsStreamIter::new(digits, 10);
    let mut max = 0;

    for words in anagrams {
        let n = words[0].len();
        permutations_iter.set_k(n);
        'perm: while let Some(slice) = permutations_iter.streaming_next() {
            word_nums.clear();
            char_to_digit.clear();

            char_to_digit.extend(words[0].chars().zip(slice.iter().cloned()) );

            if words.len() == 2 {
                for word in &words {
                    let first_ch = word.chars().next().unwrap();
                    let last_ch = word.chars().rev().next().unwrap();
                    if char_to_digit[&first_ch] == 0
                    || !([0,1,4,5,6,9].contains(&char_to_digit[&last_ch]))
                    {
                        continue 'perm
                    }
                }
            }

            word_nums.extend(
                words.iter().map(|word| {
                    word.chars().fold(0, |sum, ch| 10*sum + char_to_digit[&ch])
                })
            );
            if word_nums.iter().filter(|&&num| is_square(num)).count() >= 2 {
                let &max_num = word_nums.iter()
                    .filter(|&&num| is_square(num))
                    .max()
                    .unwrap();
                if max_num > max { max = max_num }
            }
        }
    }
    println!("{}", max);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
