use crate::error::Result;
use std::{
    collections::{BTreeSet, HashMap},
    io::BufRead,
};

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Node {
    priority: i64,
    steps: i64,
    pos: (i64, i64),
}

fn l_infty(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn pathfind(m: &Vec<Vec<u8>>, start: (i64, i64), target: (i64, i64)) -> Option<i64> {
    let mut heap = BTreeSet::new();
    let mut costs = HashMap::new();

    // let mut visited = HashSet::new();

    heap.insert(Node {
        priority: l_infty(start, target),
        steps: 0,
        pos: start,
    });
    costs.insert(start, 0);

    while let Some(item) = heap.pop_first() {
        // visited.insert(item.pos);

        if item.pos == target {
            return Some(item.steps);
        }

        let h = m[item.pos.1 as usize][item.pos.0 as usize];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let newpos = (item.pos.0 + dx, item.pos.1 + dy);
            if newpos.0 >= 0
                && (newpos.0 as usize) < m[0].len()
                && newpos.1 >= 0
                && (newpos.1 as usize) < m.len()
            {
                let h2 = m[newpos.1 as usize][newpos.0 as usize];
                let n = Node {
                    pos: newpos,
                    priority: l_infty(newpos, target) + item.steps + 1,
                    steps: item.steps + 1,
                };
                if h2 <= h + 1 {
                    if !costs.contains_key(&newpos) || costs[&newpos] > n.steps {
                        costs.insert(newpos, n.steps);
                        heap.replace(n);
                    }
                }
            }
        }
        // println!("\x1b[2J");
        // println!("{} {}", item.pos.0, item.pos.1);
        // for y in 0..m.len() {
        //     for x in 0..m[0].len() {
        //         print!(
        //             "{}{}{}",
        //             if visited.contains(&(x as i64, y as i64)) {
        //                 "\x1b[1;42m"
        //             } else {
        //                 ""
        //             },
        //             m[y][x] as char,
        //             if visited.contains(&(x as i64, y as i64)) {
        //                 "\x1b[0m"
        //             } else {
        //                 ""
        //             },
        //         );
        //     }
        //     println!("");
        // }
        // println!("{:?}", &heap);
        // println!("");
    }

    None
}

struct Input {
    m: Vec<Vec<u8>>,
    start: (i64, i64),
    target: (i64, i64),
}

fn parse_input() -> Result<Input> {
    let mut m: Vec<Vec<u8>> = Vec::new();
    let mut start = None;
    let mut target = None;
    for (y, line) in std::fs::read("inputs/day12a")?
        .trim_ascii()
        .lines()
        .enumerate()
    {
        let mut v = line?.as_bytes().to_vec();
        v.iter().position(|x| *x == 'S' as u8).map(|x| {
            start = Some((x, y));
            v[x] = 'a' as u8;
        });
        v.iter().position(|x| *x == 'E' as u8).map(|x| {
            target = Some((x, y));
            v[x] = 'z' as u8;
        });
        m.push(v);
    }
    let start = start.unwrap();
    let target = target.unwrap();
    Ok(Input {
        m,
        start: (start.0 as i64, start.1 as i64),
        target: (target.0 as i64, target.1 as i64),
    })
}

pub fn solve_a() -> Result<i64> {
    let input = parse_input()?;
    let steps = pathfind(&input.m, input.start, input.target);
    Ok(steps.unwrap())
}

pub fn solve_b() -> Result<i64> {
    let input = parse_input()?;
    let mut min_steps = i64::MAX;
    for y in 0..input.m.len() {
        for x in 0..input.m[0].len() {
            if input.m[y][x] == 'a' as u8 {
                if let Some(steps) = pathfind(&input.m, (x as i64, y as i64), input.target) {
                    min_steps = min_steps.min(steps);
                }
            }
        }
    }
    Ok(min_steps)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 447);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 446);
    }
}
