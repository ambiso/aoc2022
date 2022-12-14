use std::cmp::Ordering;

use nom::{multi::separated_list0, bytes::complete::tag, IResult, sequence::{delimited, tuple}, combinator::{map}, branch::alt, character::complete::{newline}};

use crate::{error::Result, util::parse_num};

#[derive(Debug, Clone)]
struct List(Vec<Item>);

#[derive(Debug, Clone)]
enum Item {
    List(Vec<Item>),
    Int(i64),
}

fn parse_list(i: &[u8]) -> IResult<&[u8], List> {
    delimited(tag(b"["), map(separated_list0(tag(b","), parse_item), List), tag(b"]"))(i)
}

fn parse_item(i: &[u8]) -> IResult<&[u8], Item> {
    alt((map(parse_list, |x| Item::List(x.0)), map(parse_num, Item::Int)))(i)
}

fn parse_pair(i: &[u8]) -> IResult<&[u8], (Item, Item)> {
    map(tuple((parse_list, newline, parse_list)), |(a, _, b)| (Item::List(a.0), Item::List(b.0)))(i)
}

fn parse_pairs(i: &[u8]) -> IResult<&[u8], Vec<(Item, Item)>> {
    separated_list0(tuple((newline, newline)), parse_pair)(i)
}

fn parse_packets(i: &[u8]) -> IResult<&[u8], Vec<Item>> {
    separated_list0(alt((map(tuple((newline, newline)), |_| ()), map(newline, |_| ()))), map(parse_list, |x| Item::List(x.0)))(i)
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Int(a), Item::Int(b)) => a.partial_cmp(b),
            (Item::List(a), Item::List(b)) => {
                a.partial_cmp(b)
            },
            (Item::Int(a), b) => Item::List(vec![Item::Int(*a)]).partial_cmp(b),
            (a, Item::Int(b)) => a.partial_cmp(&Item::List(vec![Item::Int(*b)])),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::List(a), Item::List(b)) => a == b,
            (Item::List(a), Item::Int(b)) => *a == vec![Item::Int(*b)],
            (Item::Int(a), Item::List(b)) => vec![Item::Int(*a)] == *b,
            (Item::Int(a), Item::Int(b)) => a == b,
        }
    }
}

impl Eq for Item {}

pub fn solve_a() -> Result<i64> {
    let f = std::fs::read("inputs/day13a")?;
    let mut idx_sum = 0;
    for (i, (l, r)) in parse_pairs(&f[..]).unwrap().1.iter().enumerate() {
        if l < r {
            idx_sum += (i+1) as i64;
        }
    }
    Ok(idx_sum)
}

pub fn solve_b() -> Result<i64> {
    let f = std::fs::read("inputs/day13a")?;
    let mut packets = parse_packets(&f[..]).unwrap().1;
    let two = Item::List(vec![Item::List(vec![Item::Int(2)])]);
    let six = Item::List(vec![Item::List(vec![Item::Int(6)])]);
    packets.push(two.clone());
    packets.push(six.clone());
    packets.sort();
    let a = packets.iter().position(|x| *x == two).unwrap() as i64 + 1;
    let b = packets.iter().position(|x| *x == six).unwrap() as i64 + 1;
    Ok(a*b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 5198);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 22344);
    }
}
