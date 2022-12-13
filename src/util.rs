use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;

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
