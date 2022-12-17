use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list0,
    sequence::tuple, IResult,
};

use crate::{
    error::Result,
    util::{parse_num, Vec2D},
};

fn parse_coord(i: &[u8]) -> IResult<&[u8], (i64, i64)> {
    map(tuple((parse_num, tag(b","), parse_num)), |(a, _, b)| (a, b))(i)
}

fn parse_coords(i: &[u8]) -> IResult<&[u8], Vec<(i64, i64)>> {
    separated_list0(tag(b" -> "), parse_coord)(i)
}

fn parse_scans(i: &[u8]) -> IResult<&[u8], Vec<Vec<(i64, i64)>>> {
    separated_list0(newline, parse_coords)(i)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Air,
    Rock,
    Sand,
}

fn solve(mut scans: Vec<Vec<(i64, i64)>>, add_floor: bool) -> i64 {
    // let min_x = scans
    //     .iter()
    //     .flat_map(|s| s.iter().map(|x| x.0).min())
    //     .min()
    //     .unwrap();
    // let max_x = scans
    //     .iter()
    //     .flat_map(|s| s.iter().map(|x| x.0).max())
    //     .max()
    //     .unwrap();
    // let min_y = scans
    //     .iter()
    //     .flat_map(|s| s.iter().map(|x| x.1).min())
    //     .min()
    //     .unwrap();
    let max_y = scans
        .iter()
        .flat_map(|s| s.iter().map(|x| x.1).max())
        .max()
        .unwrap();

    let source = (500, 0);
    if add_floor {
        scans.push(vec![
            (source.0 - 2 * max_y, 2 + max_y),
            (source.0 + 2 * max_y, 2 + max_y),
        ]);
    }

    let mut grid = Vec2D {
        v: vec![Block::Air; ((2 + max_y + 2) * (source.0 + 2 * max_y + 1)) as usize],
        stride: (source.0 + 2 * max_y + 1),
    };

    for scan in scans {
        for p in scan.windows(2) {
            if let &[(mut x1, mut y1), (mut x2, mut y2)] = p {
                if y1 == y2 {
                    if x2 < x1 {
                        std::mem::swap(&mut x1, &mut x2);
                    }

                    for x in x1..=x2 {
                        grid[(x, y1)] = Block::Rock;
                    }
                } else if x1 == x2 {
                    if y2 < y1 {
                        std::mem::swap(&mut y1, &mut y2);
                    }

                    for y in y1..=y2 {
                        grid[(x1, y)] = Block::Rock;
                    }
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        }
    }

    let mut collisions = Vec::with_capacity((max_y + 2) as usize);
    collisions.push(source);

    let mut distributed_sand = 0;
    'outer: loop {
        let mut sand_pos = match collisions.last() {
            Some(x) => *x,
            None => break,
        };

        loop {
            let mut any = false;
            for dir in [0, -1, 1] {
                if grid[(sand_pos.0 + dir, sand_pos.1 + 1)] == Block::Air {
                    sand_pos.1 += 1;
                    sand_pos.0 += dir;
                    any = true;
                    break;
                }
            }
            if !any {
                grid[sand_pos] = Block::Sand;
                collisions.pop();
                break;
            }
            collisions.push(sand_pos);
            if sand_pos.1 > max_y + 2 {
                break 'outer;
            }
        }
        distributed_sand += 1;
    }

    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         print!(
    //             "{}",
    //             match grid[x as usize][y as usize] {
    //                 Block::Air => '.',
    //                 Block::Rock => '#',
    //                 Block::Sand => 'o',
    //             }
    //         );
    //     }
    //     println!("");
    // }

    distributed_sand
}

pub fn solve_a() -> Result<i64> {
    let f = std::fs::read("inputs/day14a")?;
    let scans = parse_scans(&f[..]).unwrap().1;
    Ok(solve(scans, false))
}

pub fn solve_b() -> Result<i64> {
    let f = std::fs::read("inputs/day14a")?;
    let scans = parse_scans(&f[..]).unwrap().1;
    Ok(solve(scans, true))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 768);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 26686);
    }
}
