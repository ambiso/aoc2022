use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{collections::HashSet, io::BufRead, ops::RangeInclusive};

lazy_static! {
    static ref RE: Regex = Regex::new("=(-?[0-9]+)").unwrap();
}

use crate::{
    error::Result,
    util::{l_infty, parse_num},
};

fn parse_input() -> Result<Vec<[i64; 4]>> {
    let f = std::fs::read("inputs/day15a")?;
    Ok(f.lines()
        .filter_map(|x| x.ok())
        .map(|s| {
            let c = RE.find_iter(s.as_bytes());
            let mut v = [0i64; 4];
            v.iter_mut().zip(c).for_each(|(x, m)| {
                *x = parse_num(&m.as_bytes()[1..]).unwrap().1;
            });
            v
        })
        .collect::<Vec<_>>())
}

pub fn solve_a() -> Result<i64> {
    let input = parse_input()?;

    let mut sensor_map = HashSet::new();
    let mut beacons = HashSet::new();

    let y = 2000000;
    // let y = 10;
    for sensor in input {
        let [sx, sy, bx, by] = sensor;
        if by == y {
            beacons.insert(bx);
        }

        let d = l_infty((sx, sy), (bx, by));
        let c = (sy - y).abs();
        let ceh = d - c;

        // dbg!(sx + ceh - (sx - ceh));
        for x in sx - ceh..=sx + ceh {
            sensor_map.insert((x, y));
        }
    }

    // for x in -4..=26 {
    //     print!(
    //         "{}",
    //         if sensor_map.contains(&(x, y)) {
    //             '#'
    //         } else {
    //             '.'
    //         }
    //     );
    // }
    // println!("");

    Ok(sensor_map.len() as i64 - beacons.len() as i64)
}

trait Interval<Idx> {
    fn merge(&self, other: &RangeInclusive<Idx>) -> Option<RangeInclusive<Idx>>;
}

impl Interval<i64> for RangeInclusive<i64> {
    fn merge(&self, other: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
        // if self and other overlap -> merge
        if self.start() > self.end() || other.start() > other.end() {
            return None;
        }
        if (*self.start() <= other.end() + 1i64 && self.start() >= other.start())
            || (*self.end() + 1i64 >= *other.start() && self.start() <= other.start())
        {
            Some(*self.start().min(other.start())..=*self.end().max(other.end()))
        } else {
            None
        }
    }
}

pub fn solve_b() -> Result<i64> {
    let input = parse_input()?;

    let n = 4000001;

    let mut intervals: Vec<RangeInclusive<i64>> = vec![];

    for y in 0..n as i64 {
        intervals.clear();
        for &sensor in &input {
            let [sx, sy, bx, by] = sensor;
            let d = l_infty((sx, sy), (bx, by));
            let c = (sy - y).abs();
            let ceh = d - c;
            let mut int = (sx - ceh).max(0)..=(sx + ceh).min(n as i64 - 1);
            if int.start() <= int.end() {
                let mut i = 0;
                while i < intervals.len() {
                    if let Some(merged) = intervals[i].merge(&int) {
                        intervals.swap_remove(i);
                        int = merged;
                    } else {
                        i += 1;
                    }
                }
                intervals.push(int);
            }
        }
        if intervals.len() != 1 {
            let mut p = [
                intervals[0].start(),
                intervals[0].end(),
                intervals[1].start(),
                intervals[1].end(),
            ];
            p.sort();
            let x = p[1] + 1;
            return Ok(x as i64 * (n as i64 - 1) + y);
        }
    }

    panic!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 5256611);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 26686);
    }

    #[test]
    fn test_merge() {
        assert_eq!((0..=3).merge(&(4..=5)), Some(0..=5));
        assert_eq!((0..=4).merge(&(4..=5)), Some(0..=5));
        assert_eq!((0..=5).merge(&(4..=5)), Some(0..=5));

        assert_eq!((4..=5).merge(&(0..=3)), Some(0..=5));
        assert_eq!((4..=5).merge(&(0..=4)), Some(0..=5));
        assert_eq!((4..=5).merge(&(0..=5)), Some(0..=5));

        assert_eq!((0..=3).merge(&(5..=6)), None);
        assert_eq!((5..=6).merge(&(0..=3)), None);

        assert_eq!((0..=6).merge(&(1..=3)), Some(0..=6));
        assert_eq!((1..=3).merge(&(0..=6)), Some(0..=6));

        assert_eq!((6..=0).merge(&(4..=5)), None);
        assert_eq!((6..=0).merge(&(4..=5)), None);
        assert_eq!((6..=0).merge(&(4..=5)), None);

        assert_eq!((6..=0).merge(&(0..=3)), None);
        assert_eq!((6..=0).merge(&(0..=4)), None);
        assert_eq!((6..=0).merge(&(0..=5)), None);

        assert_eq!((6..=0).merge(&(5..=6)), None);
        assert_eq!((6..=0).merge(&(0..=3)), None);

        assert_eq!((6..=0).merge(&(1..=3)), None);
        assert_eq!((6..=0).merge(&(0..=6)), None);
    }
}
