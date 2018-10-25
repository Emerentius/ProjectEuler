// gcd(a,b) = d = s*a + t*b
// returns (d, s, t)
pub fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    // code stolen from english wikipedia
    // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Example
    let (mut s, mut old_s) = (0, 1);
    let (mut t, mut old_t) = (1, 0);
    let (mut r, mut old_r) = (b, a);
    let mut new_temp;
    while r != 0 {
        let quotient = old_r / r;
        new_temp = old_r - quotient * r;
        old_r = r;
        r = new_temp;

        new_temp = old_s - quotient * s;
        old_s = s;
        s = new_temp;

        new_temp = old_t - quotient * t;
        old_t = t;
        t = new_temp;
    }
    (old_r, old_s, old_t)
}

// a*s = 1 (mod n)
// compute s
pub fn multiplicative_inverse(a: i64, n: i64) -> Option<i64> {
    let (mut s, mut old_s) = (1, 0);
    //let (mut t, mut old_t) = (1, 0);
    let (mut r, mut old_r) = (a, n);
    let mut new_temp;
    while r != 0 {
        let quotient = old_r / r;
        new_temp = old_r - quotient * r;
        old_r = r;
        r = new_temp;

        new_temp = old_s - quotient * s;
        old_s = s;
        s = new_temp;

        //new_temp = old_t - quotient * t;
        //old_t = t;
        //t = new_temp;
    }
    if old_r > 1 { return None } // not invertible
    if old_s < 0 { old_s += n } // positive number
    Some(old_s)
}

//pub fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
//    if b == 0 { return (a, 1, 0) }
//    let (d, s, t) = extended_euclid(b, a % b);
//    (d, t, s - (a % b)*t)
//}

pub trait IntSqrt
    where Self: Sized
{
    fn isqrt(self) -> Self;
    fn sqrt(self) -> Option<Self>;
    fn is_square(self) -> bool;
}

macro_rules! implement_int_sqrt {
    ( $T:ty ) => {
        impl IntSqrt for $T {
            // newton's method
            #[inline]
            fn isqrt(self) -> Self {
                (self as f64).sqrt() as Self

                // an order of magnitude worse
                //let mut x = self;
                //let mut next_x = self / 2 + self % 2;
                //
                //while next_x < x {
                //    x = next_x;
                //    next_x = (x + self / x)/2;
                //}
                //x
                //
            }

            #[inline]
            fn sqrt(self) -> Option<Self> {
                match self % 16 {
                    0 | 1 | 4 | 9 => {
                        let root = self.isqrt();
                        match root*root == self {
                            true => Some(root),
                            false => None,
                        }
                    },
                    _ => None,
                }
            }

            #[inline]
            fn is_square(self) -> bool {
                self.sqrt().is_some()
            }
        }
    };
}

implement_int_sqrt!(u8);
implement_int_sqrt!(u16);
implement_int_sqrt!(u32);
implement_int_sqrt!(u64);
implement_int_sqrt!(usize);

// FIXME: pull into separate tests
#[test]
fn squares() {
    for i in 0u8..16 {
        let ii = i*i;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());

        let i = i as u16;
        let ii = ii as u16;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());

        let i = i as u32;
        let ii = ii as u32;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());

        let i = i as u64;
        let ii = ii as u64;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());

        let i = i as usize;
        let ii = ii as usize;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());
    }
}

// FIXME: pull into separate tests
#[test]
fn nonsquares_plus_one() {
    for i in 1u8..16 {
        let ii = i*i + 1;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());

        let i = i as u16;
        let ii = ii as u16;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());

        let i = i as u32;
        let ii = ii as u32;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());

        let i = i as u64;
        let ii = ii as u64;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());

        let i = i as usize;
        let ii = ii as usize;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());
    }
}

// FIXME: pull into separate tests
#[test]
fn nonsquares_minus_one() {
    for i in 2u8..16 {
        let ii = i*i - 1;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());

        let i = i as u16;
        let ii = ii as u16;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());

        let i = i as u32;
        let ii = ii as u32;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());

        let i = i as u64;
        let ii = ii as u64;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());

        let i = i as usize;
        let ii = ii as usize;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());
    }
}

#[test]
fn overflow() {
    (!0u8).isqrt(); // must not panic
}

#[test]
fn zero() {
    0u8.isqrt();
}
