use std::{
    collections::{BTreeSet, HashMap},
    ops::Sub,
};

use num::Signed;

use crate::{
    error::Result,
    util::{gcd, Vec2D},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
struct State {
    priority: i64,
    time: i64,
    pos: [i64; 2],
}

impl State {
    fn new(target: [i64; 2], pos: [i64; 2], time: i64) -> Self {
        Self {
            priority: l_infty(pos, target) + time,
            pos: pos,
            time: time,
        }
    }
}

pub fn l_infty<T: Copy + Signed + Sub<T> + std::iter::Sum, const N: usize>(
    a: [T; N],
    b: [T; N],
) -> T {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (*x - *y).abs())
        .sum::<T>()
}

fn print_map(map: &Vec2D<bool>, pos: [i64; 2]) {
    let (xs, ys) = map.dims();
    for y in 0..ys {
        for x in 0..xs {
            if map[(x, y)] {
                print!("x");
            } else if pos == [x, y] {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

pub fn solve_a() -> Result<i64> {
    let s = std::fs::read("inputs/day24a")?;
    let map = s
        .split(|x| *x == '\n' as u8)
        .map(Vec::from)
        .collect::<Vec<_>>();

    let blizzards = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter().enumerate().filter_map(move |(x, v)| {
                let dir = match *v as char {
                    'v' => [0i64, 1],
                    '^' => [0, -1],
                    '>' => [1, 0],
                    '<' => [-1, 0],
                    _ => return None,
                };
                Some(([x as i64, y as i64], dir))
            })
        })
        .collect::<Vec<_>>();

    let xs = map[0].len();
    let ys = map.len();
    let cycle_len = (xs * ys) as i64 / gcd((xs - 2) as _, (ys - 2) as _) as i64;
    let source = [1, 0];
    let target = [map[0].len() as i64 - 2, map.len() as i64 - 1];
    let maps = (0..cycle_len)
        .map(|t| {
            let t = t as i64;
            let mut map = Vec2D {
                v: vec![false; xs * ys],
                stride: xs as _,
            };
            for (bpos, dir) in blizzards.iter() {
                let mut new_bpos = [bpos[0] + dir[0] * t, bpos[1] + dir[1] * t];
                new_bpos[0] = ((new_bpos[0] - 1).rem_euclid(xs as i64 - 2)) + 1;
                new_bpos[1] = ((new_bpos[1] - 1).rem_euclid(ys as i64 - 2)) + 1;
                map[(new_bpos[0], new_bpos[1])] = true;
            }
            // print_map(&map, target);
            // println!("");
            map
        })
        .collect::<Vec<_>>();

    let mut frontier = BTreeSet::<State>::new();
    frontier.insert(State::new(target, source, 0));

    let mut backlinks = HashMap::new();

    while let Some(s) = frontier.pop_first() {
        if s.pos == target {
            let t = s.time;
            let mut s = s;
            let mut path = Vec::new();
            while let Some(x) = backlinks.get(&s) {
                path.push(*x);
                s = *x;
            }
            path.reverse();
            for s in path.iter() {
                let map = &maps[s.time as usize % maps.len()];
                print_map(map, s.pos);
                println!("");
            }
            return Ok(t);
        }
        let map = &maps[(s.time + 1) as usize % maps.len()];

        for dir in [[0, 0], [-1, 0], [1, 0], [0, -1], [0, 1]] {
            let pos = [s.pos[0] + dir[0], s.pos[1] + dir[1]];
            if (pos[0] > 0 && pos[1] > 0 && pos[0] < xs as i64 - 1 && pos[1] < ys as i64 - 1)
                || pos == source
                || pos == target
            {
                if !map[(pos[0], pos[1])] {
                    let s_prime = State::new(target, pos, s.time + 1);
                    backlinks.entry(s_prime).and_modify(|x| *x = s).or_insert(s);
                    frontier.insert(s_prime);
                }
            }
        }
    }

    panic!()
}
pub fn solve_b() -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 3877);
    }
    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 982);
    }
}
