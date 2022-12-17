use crate::error::Result;
use crate::util::l_infty;
use std::{
    collections::{BTreeSet, HashMap},
    io::BufRead,
};

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Copy, Clone)]
struct Node {
    priority: i64,
    steps: i64,
    pos: (i64, i64),
}

fn pathfind(
    m: &Vec<Vec<u8>>,
    start: (i64, i64),
    target: impl Fn(Node) -> bool,
    target_heuristic: impl Fn((i64, i64), Node) -> i64,
    height_check_inverse: bool,
) -> Option<i64> {
    let mut heap = BTreeSet::new();

    let mut costs = HashMap::new();
    // let mut visited = HashSet::new();

    heap.insert(Node {
        priority: 0,
        steps: 0,
        pos: start,
    });
    costs.insert(start, 0);

    while let Some(item) = heap.pop_first() {
        // visited.insert(item.pos);

        if target(item) {
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
                    priority: target_heuristic(newpos, item),
                    steps: item.steps + 1,
                };
                let mut h = h;
                let mut h2 = h2;
                if height_check_inverse {
                    std::mem::swap(&mut h, &mut h2);
                }
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
    let steps = pathfind(
        &input.m,
        input.start,
        |x| x.pos == input.target,
        |newpos, item| l_infty(newpos, input.target) + item.steps + 1,
        false,
    );
    Ok(steps.unwrap())
}

pub fn solve_b() -> Result<i64> {
    let input = parse_input()?;
    Ok(pathfind(
        &input.m,
        input.target,
        |n| input.m[n.pos.1 as usize][n.pos.0 as usize] == 'a' as u8,
        |_newpos, item| item.steps,
        true,
    )
    .unwrap())
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
