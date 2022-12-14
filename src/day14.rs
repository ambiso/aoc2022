use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{eof, iterator, map, ParserIterator, opt},
    multi::separated_list0,
    sequence::{tuple, terminated},
    IResult,
};

use crate::{error::Result, util::parse_num};

fn parse_coord(i: &[u8]) -> IResult<&[u8], (i64, i64)> {
    map(tuple((parse_num, tag(b","), parse_num)), |(a, _, b)| (a, b))(i)
}

// fn parse_coords<F, E>(i: &[u8]) -> ParserIterator<&[u8], , fn(&[u8]) -> IResult<&[u8], (i64, i64)>>
// {
//     // separated_list0(tag(b" -> "), parse_coord)(i)
//     iterator(i, tuple((parse_coord, tag(b" -> "))))
// }

// fn parse_scans(i: &[u8]) -> IResult<&[u8], Vec<Vec<(i64, i64)>>> {
//     // separated_list0(newline, parse_coords)(i)
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Air,
    Rock,
    // Sand,
}

fn solve(add_floor: bool) -> Result<i64> {
    let f = std::fs::read("inputs/day14a")?;

    let mut max_y = i64::MIN;

    // let mut grid =
    //     vec![vec![Block::Air; (2 + max_y + 2) as usize]; (source.0 + 2 * max_y + 1) as usize];
    let mut grid = vec![vec![Block::Air; 200]; 1000];

    for scan in &mut iterator(
        &f[..],
        terminated(
            |i| {
                let mut last = None;
                let mut it = iterator(i, terminated(parse_coord, opt(tag(b" -> "))));
                for (x2, y2) in &mut it {
                    // dbg!(x2, y2);
                    max_y = max_y.max(y2);
                    if let Some((x1, y1)) = last {
                        if y1 == y2 {
                            let mut x1 = x1;
                            let mut x2 = x2;
                            if x2 < x1 {
                                std::mem::swap(&mut x1, &mut x2);
                            }

                            for x in x1..=x2 {
                                grid[x as usize][y1 as usize] = Block::Rock;
                            }
                        } else if x1 == x2 {
                            let mut y1 = y1;
                            let mut y2 = y2;
                            if y2 < y1 {
                                std::mem::swap(&mut y1, &mut y2);
                            }

                            for y in y1..=y2 {
                                grid[x1 as usize][y as usize] = Block::Rock;
                            }
                        } else {
                            panic!()
                        }
                    }
                    last = Some((x2, y2));
                }
                it.finish()
            },
            tag::<_,_,nom::error::Error<&[u8]>>(b"\n"),
        ),
    ) {
    }

    let source = (500, 0);

    // if add_floor {
    //     scans.push(vec![
    //         (source.0 - 2 * max_y, 2 + max_y),
    //         (source.0 + 2 * max_y, 2 + max_y),
    //     ]);
    // }

    let mut distributed_sand = 0;
    'outer: loop {
        let mut sand_pos = source;

        if grid[sand_pos.0 as usize][sand_pos.1 as usize] == Block::Rock {
            break;
        }

        loop {
            if grid[sand_pos.0 as usize][sand_pos.1 as usize + 1] == Block::Air {
                sand_pos.1 += 1;
            } else if grid[sand_pos.0 as usize - 1][sand_pos.1 as usize + 1] == Block::Air {
                sand_pos.1 += 1;
                sand_pos.0 -= 1;
            } else if grid[sand_pos.0 as usize + 1][sand_pos.1 as usize + 1] == Block::Air {
                sand_pos.1 += 1;
                sand_pos.0 += 1;
            } else {
                grid[sand_pos.0 as usize][sand_pos.1 as usize] = Block::Rock;
                break;
            }
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

    Ok(distributed_sand)
}

pub fn solve_a() -> Result<i64> {
    solve(false)
}

pub fn solve_b() -> Result<i64> {
    solve(true)
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
