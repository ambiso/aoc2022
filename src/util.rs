use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt};
use nom::sequence::tuple;

use crate::error::Result;
use std::fs;
use std::path::Path;
pub fn read_string(path: impl AsRef<Path>) -> Result<String> {
    Ok(String::from_utf8(fs::read(path)?)?)
}

pub fn parse_num(i: &[u8]) -> IResult<&[u8], i64> {
    map(
        |i| tuple((opt(tag(b"-")), digit1))(i),
        |(sign, x): (_, &[u8])| {
            let mut acc: i64 = 0;
            for d in x.iter() {
                acc *= 10;
                acc += (d - '0' as u8) as i64;
            }
            acc * if sign.is_some() { -1 } else { 1 }
        },
    )(i)
}

// From wikipedia/uutils
pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    use std::cmp::min;
    use std::mem::swap;

    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both u and v
    let i = u.trailing_zeros();  u >>= i;
    let j = v.trailing_zeros();  v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} is even", u);
        debug_assert!(v % 2 == 1, "v = {} is even", v);

        // Swap if necessary so u <= v
        if u > v {
            swap(&mut u, &mut v);
        }
        // u and v are still both odd after (potentially) swapping

        // Using identity 4 (gcd(u, v) = gcd(|v-u|, min(u, v))
        v -= u;
        // v is now even, but u is unchanged (and odd)

        // Identity 1: gcd(u, 0) = u
        // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
        if v == 0 {
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) (u is known to be odd)
        v >>= v.trailing_zeros();
        // v is now odd again
    }
}
