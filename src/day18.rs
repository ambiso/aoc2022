use std::collections::{BTreeSet, HashSet};

use crate::error::Result;
pub fn solve_a() -> Result<i64> {
    let s = String::from_utf8(std::fs::read("inputs/day18a")?)?;
    let mut map = [[[false; 22]; 22]; 22];
    let mut coords = Vec::new();
    for l in s.lines() {
        let mut v = [0i8; 3];
        l.split(",")
            .filter_map(|x| x.parse().ok())
            .enumerate()
            .for_each(|(i, x)| {
                v[i] = x;
            });
        coords.push(v);
        map[v[0] as usize][v[1] as usize][v[2] as usize] = true;
    }
    let mut surface = 0;
    for c in coords {
        for d in [-1, 1] {
            for dir in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let dir = (dir.0 * d, dir.1 * d, dir.2 * d);
                let p = (c[0] + dir.0, c[1] + dir.1, c[2] + dir.2);
                if map
                    .get(p.0 as usize)
                    .and_then(|v| v.get(p.1 as usize))
                    .and_then(|v| v.get(p.2 as usize))
                    .map(|x| !x)
                    .unwrap_or(true)
                {
                    surface += 1;
                }
            }
        }
    }
    Ok(surface)
}
pub fn solve_b() -> Result<i64> {
    let s = String::from_utf8(std::fs::read("inputs/day18a")?)?;
    let mut map = [[[false; 22]; 22]; 22];
    let mut reachable = map.clone();
    let mut coords = Vec::new();
    for l in s.lines() {
        let mut v = [0i8; 3];
        l.split(",")
            .filter_map(|x| x.parse().ok())
            .enumerate()
            .for_each(|(i, x)| {
                v[i] = x;
            });
        coords.push(v);
        map[v[0] as usize][v[1] as usize][v[2] as usize] = true;
    }

    let mut q = BTreeSet::new();
    let mut visited = HashSet::new();
    q.insert((0, 0, 0));
    assert!(!map[0][0][0]);

    while let Some(n) = q.pop_first() {
        visited.insert(n);
        reachable[n.0 as usize][n.1 as usize][n.2 as usize] = true;
        for d in [-1i32, 1] {
            for dir in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let dir = (dir.0 * d, dir.1 * d, dir.2 * d);
                let p = (n.0 + dir.0, n.1 + dir.1, n.2 + dir.2);
                if map
                    .get(p.0 as usize)
                    .and_then(|v| v.get(p.1 as usize))
                    .and_then(|v| v.get(p.2 as usize))
                    .map(|x| !x)
                    .unwrap_or(false)
                    && !visited.contains(&p)
                {
                    q.insert(p);
                }
            }
        }
    }

    dbg!(reachable);

    let mut surface = 0;
    for c in coords {
        for d in [-1, 1] {
            for dir in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let dir = (dir.0 * d, dir.1 * d, dir.2 * d);
                let p = (c[0] + dir.0, c[1] + dir.1, c[2] + dir.2);
                if map
                    .get(p.0 as usize)
                    .and_then(|v| v.get(p.1 as usize))
                    .and_then(|v| v.get(p.2 as usize))
                    .map(|x| !x)
                    .unwrap_or(true)
                    && reachable
                        .get(p.0 as usize)
                        .and_then(|v| v.get(p.1 as usize))
                        .and_then(|v| v.get(p.2 as usize))
                        .map(|x| *x)
                        .unwrap_or(true)
                {
                    surface += 1;
                }
            }
        }
    }
    Ok(surface)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 3232);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_a().unwrap(), 1585632183915);
    }
}
