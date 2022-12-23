use std::collections::{HashMap, VecDeque, HashSet};

use crate::error::Result;

pub fn solve_a() -> Result<i64> {
    let s = std::fs::read("inputs/day23a")?;
//     let s = b".....
// ..##.
// ..#..
// .....
// ..##.
// .....";
    let mut map = s
        .split(|x| *x == '\n' as u8)
        .map(Vec::from)
        .collect::<Vec<_>>();
    map.iter_mut().for_each(|line| {
        line.iter_mut().for_each(|v| *v = (*v == '#' as u8) as u8);
    });

    let mut positions = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 1 {
                positions.push([x as isize, y as isize]);
            }
        }
    }

    let mut directions = VecDeque::from([
        [[0, -1], [-1, -1], [1, -1]],
        [[0, 1], [-1, 1], [1, 1]],
        [[-1, 0], [-1, -1], [-1, 1]],
        [[1, 0], [1, -1], [1, 1]],
    ]);


    for _ in 0..10 {
        let mut proposals = Vec::new();
        for elf in positions.iter() {
            let mut has_proposal = false;


            if !positions.iter().filter(|p| *p != elf).any(|p| (elf[0] - p[0]).abs() <= 1 && (elf[1] - p[1]).abs() <= 1) {
                proposals.push(None);
                continue
            }

            for direction in directions.iter() {
                let mut invalid = false;
                for offset_to_check in direction {
                    let new_pos = [elf[0] + offset_to_check[0], elf[1] + offset_to_check[1]];
                    if positions.iter().any(|x| *x == new_pos) {
                        invalid = true;
                        break;
                    }
                }
                if !invalid {
                    let new_pos = [elf[0] + direction[0][0], elf[1] + direction[0][1]];
                    proposals.push(Some(new_pos));
                    has_proposal = true;
                    break;
                }
            }
            if !has_proposal {
                proposals.push(None);
            }
        }

        let mut proposal_counts = HashMap::<_, i64>::new();

        for prop in proposals.iter() {
            if let Some(p) = prop {
                *proposal_counts.entry(*p).or_default() += 1;
            }
        }

        for (pos, prop) in positions.iter_mut().zip(proposals.iter()) {
            if let Some(p) = prop {
                if proposal_counts[p] == 1 {
                    *pos = *p;
                }
            }
        }

        let f = directions.pop_front().unwrap();
        directions.push_back(f);


        // for y in min_y..=max_y {
        //     for x in min_x..=max_x {
        //         let c = if positions.iter().any(|p| *p == [x as isize, y as isize]) {
        //             '#'
        //         } else {
        //             '.'
        //         };

        //         print!("{c}");
        //     }
        //     println!("");
        // }
        // println!("");
    }

    let min_x = positions.iter().map(|p| p[0]).min().unwrap();
    let max_x = positions.iter().map(|p| p[0]).max().unwrap();
    let min_y = positions.iter().map(|p| p[1]).min().unwrap();
    let max_y = positions.iter().map(|p| p[1]).max().unwrap();
    let r = (max_x - min_x + 1) * (max_y - min_y + 1);
    Ok(r as i64 - positions.len() as i64)
}

pub fn solve_b() -> Result<i64> {
    let s = std::fs::read("inputs/day23a")?;
//     let s = b".....
// ..##.
// ..#..
// .....
// ..##.
// .....";
    let mut map = s
        .split(|x| *x == '\n' as u8)
        .map(Vec::from)
        .collect::<Vec<_>>();
    map.iter_mut().for_each(|line| {
        line.iter_mut().for_each(|v| *v = (*v == '#' as u8) as u8);
    });

    let mut hashgrid = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 1 {
                hashgrid.insert([x as isize, y as isize]);
            }
        }
    }

    let mut directions = VecDeque::from([
        [[0, -1], [-1, -1], [1, -1]],
        [[0, 1], [-1, 1], [1, 1]],
        [[-1, 0], [-1, -1], [-1, 1]],
        [[1, 0], [1, -1], [1, 1]],
    ]);


    for round in 0..{
        let mut proposals = HashMap::new();
        for elf in hashgrid.iter() {
            let mut any_in_vicinity = false;
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx != 0 || dy != 0 {
                        if hashgrid.contains(&[elf[0] + dx, elf[1] + dy]) {
                            any_in_vicinity = true;
                        }
                    }
                }
            }
            if !any_in_vicinity {
                continue
            }

            for direction in directions.iter() {
                let mut invalid = false;
                for offset_to_check in direction {
                    let new_pos = [elf[0] + offset_to_check[0], elf[1] + offset_to_check[1]];
                    if hashgrid.contains(&new_pos) {
                        invalid = true;
                        break;
                    }
                }
                if !invalid {
                    let new_pos = [elf[0] + direction[0][0], elf[1] + direction[0][1]];
                    proposals.insert(*elf, new_pos);
                    break;
                }
            }
        }

        let mut proposal_counts = HashMap::<_, i64>::new();

        for (_, prop) in proposals.iter() {
            *proposal_counts.entry(*prop).or_default() += 1;
        }

        let mut any_moved = false;
        let positions: Vec<[isize; 2]> = hashgrid.iter().map(|x| *x).collect::<Vec<_>>();
        for pos in positions {
            if let Some(p) = proposals.get(&pos) {
                if proposal_counts[p] == 1 {
                    any_moved = true;
                    hashgrid.remove(&pos);
                    hashgrid.insert(*p);
                }
            }
        }

        let f = directions.pop_front().unwrap();
        directions.push_back(f);


        if !any_moved {
            return Ok(round + 1);
        }

    }

    panic!()
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
