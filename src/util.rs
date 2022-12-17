use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;
use std::ops::{Index, IndexMut};

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

pub fn l_infty(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

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
        (self.stride - 1 as i64, self.v.len() as i64 / self.stride)
    }
}
