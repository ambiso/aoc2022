use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;
use std::ops::{Index, IndexMut};
use std::time::Duration;

use crate::error::Result;
use std::fs;
use std::path::Path;
pub fn read_string(path: impl AsRef<Path>) -> Result<String> {
    Ok(String::from_utf8(fs::read(path)?)?)
}

pub fn format_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{:.02}s ", d.as_millis() as f64 / 1000.0)
    } else if d.as_millis() > 0 {
        format!("{:.02}ms", d.as_nanos() as f64 / 1000.0 / 1000.0)
    } else {
        format!("{:.02}µs", d.as_nanos() as f64 / 1000.0)
    }
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

pub fn l_infty(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(PartialEq, Eq, Debug)]
pub struct Vec2D<T> {
    pub v: Vec<T>,
    pub stride: i64,
}

impl<T> Index<(i64, i64)> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        &self.v[(index.0 + index.1 * self.stride) as usize]
    }
}

impl<T> IndexMut<(i64, i64)> for Vec2D<T> {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut T {
        &mut self.v[(index.0 + index.1 * self.stride) as usize]
    }
}

impl<T> Vec2D<T> {
    pub fn dims(&self) -> (i64, i64) {
        (self.stride as i64, self.v.len() as i64 / self.stride)
    }
}

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
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
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
