extern crate bit_vec;
extern crate primal;

use crate::num::IntSqrt;

use self::bit_vec::BitVec;
use std::ops::Index;

pub trait DivisorExt {
    fn n_divisors(&self, num: usize) -> Option<usize>;

    /// Number of divisors of `num.pow(exp)`
    fn n_divisors_pow(&self, num: usize, exp: usize) -> Option<usize>;

    fn divisors(&self, num: usize) -> Option<Vec<usize>>;
}

impl DivisorExt for primal::Sieve {
    fn n_divisors(&self, num: usize) -> Option<usize> {
        self.factor(num).ok().map(|factors| {
            factors
                .into_iter()
                .fold(1, |prod, (_, occ)| prod * (occ + 1))
        })
    }

    fn n_divisors_pow(&self, num: usize, exp: usize) -> Option<usize> {
        self.factor(num).ok().map(|factors| {
            factors
                .into_iter()
                .fold(1, |prod, (_, occ)| prod * (occ * exp + 1))
        })
    }

    fn divisors(&self, num: usize) -> Option<Vec<usize>> {
        let Ok(factors) = self.factor(num) else { return None };
        let mut divisors = if num == 1 { vec![1] } else { vec![1, num] };
        visit_divisors(num, num.isqrt(), &factors, 1, &mut divisors);
        divisors.sort();
        Some(divisors)
    }
}

fn visit_divisors(
    num: usize,
    num_isqrt: usize,
    factors: &[(usize, usize)],
    current_div: usize,
    divisors: &mut Vec<usize>,
) {
    if let Some((&(prime, multiplicity), rest)) = factors.split_first() {
        let mut next_div = current_div;

        visit_divisors(num, num_isqrt, rest, next_div, divisors);

        for _ in 0..multiplicity {
            next_div *= prime;

            if next_div > num_isqrt {
                break;
            }

            divisors.push(next_div);
            let paired_div = num / next_div;
            if paired_div != next_div {
                divisors.push(paired_div);
            }
            visit_divisors(num, num_isqrt, rest, next_div, divisors);
        }
    }
}

pub struct IsPrime {
    is_prime: BitVec,
}

impl Index<usize> for IsPrime {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            2 => &self.is_prime[1], // = is_prime(3) = true, only even prime
            _ if index % 2 == 0 => &self.is_prime[0], // = is_prime(1) = false, all evens
            _ => &self.is_prime[(index - 1) / 2],
        }
    }
}

pub struct PrimeIter {
    is_prime: BitVec,
    first: bool,
    index: usize,
}

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(2);
        }

        // find next prime
        loop {
            if self.index == self.len() {
                return None;
            }
            if self.is_prime[self.index] {
                break;
            }
            self.index += 1;
        }

        let pr = self.index * 2 + 1;
        // step ahead for next time
        self.index += 1;

        // sieve multiples away
        for i in (pr * pr..2 * self.len() + 1).step_by(2 * pr) {
            self.is_prime.set((i - 1) / 2, false);
        }

        Some(pr as u64)
    }
}

impl PrimeIter {
    pub fn to_sieve(mut self) -> IsPrime {
        self.drain();
        IsPrime {
            is_prime: self.is_prime,
        }
    }

    fn drain(&mut self) {
        while self.next().is_some() {}
    }

    fn len(&self) -> usize {
        self.is_prime.len()
    }
}

/// Returns a PrimeIter, which generates primes on the go from a sieve.
/// This allows for about one order of magnitude higher limits
pub fn prime_iter(max_prime: usize) -> PrimeIter {
    let mut primes = PrimeIter {
        is_prime: BitVec::from_elem((1 + max_prime) / 2, true),
        index: 1,
        first: true,
    };
    primes.is_prime.set(0, false); // 1
                                   //primes.is_prime.set(1, false);

    primes
}

pub fn erat_sieve(max_prime: usize) -> Vec<u64> {
    let mut is_prime = BitVec::from_elem(max_prime + 1, true);
    //let mut is_prime = vec![true; max_prime+1];
    //is_prime[0]=false;
    //is_prime[1]=false;
    is_prime.set(0, false);
    is_prime.set(1, false);

    let factor = if max_prime < 10_000 {
        1.25
    } else if max_prime < 1_000_000 {
        1.15
    } else {
        1.1
    };

    //let factor = 1.;
    let estimated_prime_count = (max_prime as f64 / (max_prime as f64).ln() * factor) as usize;

    let mut prime_numbers: Vec<u64> = vec![];
    prime_numbers.reserve(estimated_prime_count);

    for number in 0..is_prime.len() {
        if is_prime[number] {
            prime_numbers.push(number as u64);
            for i in (number * number..max_prime + 1).step_by(number) {
                is_prime.set(i, false);
                //is_prime[i] = false;
            }
        }
    }
    prime_numbers
}

fn neg(bits: &mut BitVec, pos: usize) {
    let val = bits[pos];
    bits.set(pos, !val);
}

/// `sieve` returns a vector of all primes
/// from 2 up to the given number
///
/// # Arguments
///
/// * `max_prime` - The number up to which you want primes to be generated (including max_prime)
///
/// # Example
/// ```rust
/// let upper_bound : usize = 1_000_000;
/// let primes = euler_utils::prime::sieve(upper_bound);
/// ```
pub fn atkin_sieve(max_prime: usize) -> Vec<u64> {
    let mut is_prime = BitVec::from_elem(max_prime + 1, false);

    let estimated_prime_count = (max_prime as f64 / (max_prime as f64).ln() * 1.3) as usize;

    let mut primes: Vec<u64> = vec![2, 3, 5];
    primes.reserve(estimated_prime_count);

    // 4x^2 + y^2 = n
    //for x in 1..sqrt_int(max_prime)/2 {
    for x in 1.. {
        let _4xx = 4 * x * x;
        if _4xx > max_prime {
            break;
        }
        for y in (1..).step_by(2) {
            let n = _4xx + y * y;
            if n > max_prime {
                break;
            }
            match n % 60 {
                1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 => neg(&mut is_prime, n),
                _ => {}
            };
        }
    }
    // 3x^2 + y^2 = n
    for x in (1..).step_by(2) {
        let _3xx = 3 * x * x;
        if _3xx > max_prime {
            break;
        }
        for y in (2..).step_by(2) {
            let n = _3xx + y * y;
            if n > max_prime {
                break;
            }
            match n % 60 {
                7 | 19 | 31 | 43 => neg(&mut is_prime, n),
                _ => {}
            }
        }
    }
    // 3x^2 - y^2 = n
    for x in 1.. {
        let xx = x * x;
        let _3xx = 3 * xx;
        if 2 * xx + 2 * x - 1 > max_prime {
            break;
        }
        // step_by() and rev() not compatible
        let x_even = x % 2 == 0;
        for y in (1..x).rev() {
            if (x_even && y % 2 == 0) || (!x_even && y % 2 != 0) {
                continue;
            }
            let n = _3xx - y * y;
            if n > max_prime {
                break;
            }
            match n % 60 {
                11 | 23 | 47 | 59 => neg(&mut is_prime, n),
                _ => {}
            }
        }
    }

    for i in 0..is_prime.len() {
        if !is_prime[i] {
            continue;
        }
        primes.push(i as u64);

        let ii = i * i;
        for j in (ii..max_prime + 1).step_by(2 * ii) {
            is_prime.set(j, false);
        }
    }
    primes
}

pub fn sieve(max_prime: usize) -> Vec<u64> {
    prime_iter(max_prime).collect()
}

/// Calculate prime factors by trial division and return vector of results
/// Returns Vec<[prime, occurences]>
pub fn factors(mut to_factorise: u64, primes: &[u64]) -> Vec<[u64; 2]> {
    let mut factors = vec![];
    for &prime in primes {
        if prime * prime > to_factorise {
            factors.push([to_factorise, 1]);
            break;
        }
        let mut occurences = 0;
        while to_factorise % prime == 0 {
            to_factorise /= prime;
            occurences += 1;
        }
        if occurences > 0 {
            factors.push([prime, occurences])
        }
        if to_factorise == 1 {
            break;
        }
    }
    factors
}

/// check primality by trial division
pub fn is_prime_trial_div(number: u64, primes: &[u64]) -> bool {
    if number == 1 {
        return false;
    }
    if number == 2 {
        return true;
    } // could slip through sqrt case
    let sqrt_num = (number as f64).sqrt() as u64 + 1;

    let nr_primes = primes.len();
    let last_prime = if nr_primes != 0 {
        primes[primes.len() - 1]
    } else {
        3
    };

    for &prime in primes.iter() {
        if prime > sqrt_num {
            break;
        };
        if number % prime == 0 {
            return false;
        };
    }

    for n in (last_prime..sqrt_num).step_by(2) {
        if number % n == 0 {
            return false;
        };
    }
    true
}

/// Primality test by Miller-Rabin.
/// Deterministic for u64
/// but may panic for number > 2^32
pub fn is_prime(number: u64) -> bool {
    if number == 1 {
        return false;
    }

    // some trial divisions to find most non-primes already
    // create helper function later
    let trial_primes = [2, 3, 5, 7, 11, 13, 17, 19]; //23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163];
    for &prime in trial_primes.iter() {
        if number % prime == 0 {
            if number == prime {
                return true;
            }
            return false;
        }
    }

    let bases = match number {
        0..=1_373_652 => vec![2, 3],
        1_373_653..=2_152_302_898_746 => vec![2, 3, 5, 7, 11],
        _ => vec![2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022],
    };
    let number_min = number - 1;
    let j = (number_min).trailing_zeros();
    let d_u64 = (number_min) >> j; // same as (number - 1)/ 2^j
    let d = if d_u64 > u32::MAX as u64 {
        panic!("uneven factor of (number - 1) larger than 2^32 - 1 , arithmetic overflow")
    } else {
        d_u64 as u32
    };

    'base: for &base in &bases {
        if base >= number {
            break;
        }
        let mut num = crate::num::pow_mod(base, d, number);
        // possibly prime
        if num == 1 || num == number_min {
            continue 'base;
        }
        for _ in 0..j {
            num = (num * num) % number;
            if num == 1 {
                return false;
            }
            // possibly prime
            if num == number_min {
                continue 'base;
            }
        }
        // definitely composite
        return false;
    }
    true
}

// Iterator that returns all fractions between 0 and 1
// for a given max denominator
pub struct FareySequence {
    lst: (usize, usize),
    cur: (usize, usize),
    max_denom: usize,
}

impl FareySequence {
    pub fn new(n: usize) -> FareySequence {
        FareySequence {
            lst: (0, 1),
            cur: (1, n),
            max_denom: n,
        }
    }
}

impl Iterator for FareySequence {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == (1, 1) {
            return None;
        }

        // a common factor
        let num = (self.max_denom + self.lst.1) / self.cur.1;

        let next = (num * self.cur.0 - self.lst.0, num * self.cur.1 - self.lst.1);
        self.lst = self.cur;
        self.cur = next;
        Some(self.lst)
    }
}

// // maybe switch to cell?
// /// A struct to save the totients for all numbers up to a given maximum
// pub struct Phi {
//     totients: Vec<usize>,
// }

// pub type PhiIter<'a> = std::iter::Skip<std::slice::Iter<'a, usize>>;

// impl Phi {
//     pub fn new(max: usize) -> Phi {
//         let mut totients: Vec<_> = (0..max + 1).collect();
//         for i in 2..max + 1 {
//             if totients[i] == i {
//                 for n in (i..max + 1).step_by(i) {
//                     totients[n] -= totients[n] / i;
//                 }
//             }
//         }
//         Phi { totients }
//     }

//     pub fn iter(&self) -> PhiIter<'_> {
//         self.totients.iter().skip(1)
//     }
// }

// impl Index<usize> for Phi {
//     type Output = usize;

//     fn index(&self, idx: usize) -> &Self::Output {
//         if idx == 0 {
//             panic!("Totients are only defined for positive integers")
//         }
//         &self.totients[idx]
//     }
// }

// maybe switch to cell?
/// A struct to save the totients for all numbers up to a given maximum
/// using 32 bit integers to save space
pub struct Phi32 {
    totients: Vec<u32>,
}

pub type Phi32Iter<'a> = std::iter::Skip<std::slice::Iter<'a, u32>>;

impl Phi32 {
    pub fn new(max: u32) -> Phi32 {
        let mut totients: Vec<_> = (0..max + 1).collect();
        for i in 2..max + 1 {
            // φ is multiplicative, so φ(n) = φ(p1^k * p2^l...) = φ(p1^k) * φ(p2^l) … .
            // for p prime, φ(p^k) = p^k * (1 - 1/p)
            // iterate through all primes p and subtract the previous value of totient[n]
            // from totient[n] for all n that are multiples of p.
            // That's equivalent to multiplying with (1 - 1/p)
            //
            // if nothing prior has reduced totients[i], then i is prime
            if totients[i as usize] == i {
                for n in (i..max + 1).step_by(i as usize) {
                    let n = n as usize;
                    assert!(totients[n] % i == 0);
                    totients[n] -= totients[n] / i;
                }
            }
        }
        Phi32 { totients }
    }

    pub fn iter(&self) -> Phi32Iter<'_> {
        self.totients.iter().skip(1)
    }
}

impl Index<u32> for Phi32 {
    type Output = u32;

    fn index(&self, idx: u32) -> &Self::Output {
        if idx == 0 {
            panic!("Totients are only defined for positive integers")
        }
        &self.totients[idx as usize]
    }
}

#[test]
fn test_totients() {
    let phi = Phi32::new(36);
    let phi = phi.iter().copied().collect::<Vec<_>>();
    let expected_phi = vec![
        1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20, 12, 18,
        12, 28, 8, 30, 16, 20, 16, 24, 12,
    ];
    assert_eq!(phi, expected_phi);
}

#[test]
fn test_divisors() {
    let sieve = primal::Sieve::new(1000);

    fn naive_divisors(num: usize) -> Vec<usize> {
        (1..num + 1).filter(|d| num % d == 0).collect()
    }

    for n in 1..1_000 {
        assert_eq!(sieve.divisors(n).unwrap(), naive_divisors(n));
    }
}
