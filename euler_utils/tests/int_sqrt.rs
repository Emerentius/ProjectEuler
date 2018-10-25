extern crate euler_utils;
use euler_utils::num::IntSqrt;

#[test]
fn isqrt_correctness() {
    for n in 1u32..1_000 {
        let sq = n*n;
        let next_sq = (n+1)*(n+1);
        for m in sq..next_sq {
            assert!( m.isqrt() == n);
        }
    }
}

#[test]
#[ignore]
fn isqrt_exhaustive_correctness_u32() {
    for n in 0u32..2u32.pow(16) {
        let sq = n*n;
        let next_sq = (n+1).saturating_mul(n+1);
        for m in sq..next_sq {
            assert!( m.isqrt() == n, "n: {}, n²: {}, m: {}", n, sq, m);
        }
    }
}

#[test]
fn sqrt_correctness() {
    for n in 1u32..1_000 {
        let sq = n*n;
        let next_sq = (n+1)*(n+1);

        assert!( sq.sqrt().is_some() );

        for m in sq+1..next_sq {
            assert!( m.sqrt().is_none() );
        }
    }
}

#[test]
fn squareness_check() {
    let squares: Vec<u32> = (1..1000).map(|n| n*n).collect();
    for &n_sq in &squares {
        assert!(n_sq.is_square());
    }
    for n_non_sq in (1..1_000_000).filter(|n| squares.binary_search(&n).is_err()) {
        assert!( !n_non_sq.is_square() );
    }
}
