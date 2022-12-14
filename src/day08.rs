use crate::error::Result;
use crate::util::Vec2D;

fn parse_input() -> Result<Vec2D<u8>> {
    let mut v = std::fs::read("inputs/day08a")?;
    let mut stride = 0;
    for (i, x) in v.iter_mut().enumerate() {
        if *x == '\n' as u8 {
            if stride == 0 {
                stride = i + 1;
            }
        } else {
            *x -= '0' as u8;
        }
    }
    Ok(Vec2D {
        v,
        stride: stride as i64,
    })
}

fn is_visible(p: (i64, i64), v: &Vec2D<u8>) -> bool {
    let (xm, ym) = v.dims();
    let height = v[p];
    'next_dir: for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut pn = p;
        loop {
            pn = (pn.0 + dx, pn.1 + dy);
            if pn.0 >= 0 && pn.0 < xm && pn.1 >= 0 && pn.1 < ym {
                if v[pn] >= height {
                    continue 'next_dir;
                }
            } else {
                return true; // reached an edge
            }
        }
    }
    false
}

fn scenic_score(p: (i64, i64), v: &Vec2D<u8>) -> i64 {
    let (xm, ym) = v.dims();
    let height = v[p];
    let mut score = 1;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut visible = 0;
        let mut pn = p;
        loop {
            pn = (pn.0 + dx, pn.1 + dy);
            if pn.0 >= 0 && pn.0 < xm && pn.1 >= 0 && pn.1 < ym {
                visible += 1;
                if v[pn] >= height {
                    break;
                }
            } else {
                break;
            }
        }
        score *= visible;
    }
    score
}

pub fn solve_a() -> Result<u64> {
    let v = parse_input()?;

    let (xm, ym) = v.dims();

    let mut visible = 0;
    for x in 0..xm {
        for y in 0..ym {
            if is_visible((x, y), &v) {
                visible += 1;
            }
        }
    }

    Ok(visible)
}

pub fn solve_a_opt() -> Result<u64> {
    let v = parse_input()?;

    let (xm, ym) = v.dims();

    let mut vis = Vec2D {
        v: vec![false; (xm * ym) as usize],
        stride: xm,
    };

    let mut visible = 0;

    for x in 0..xm {
        visible += do_scan(&v, &mut vis, x, 0, 0, 1);
        visible += do_scan(&v, &mut vis, x, xm - 1, 0, -1);
    }

    for y in 0..ym {
        visible += do_scan(&v, &mut vis, 0, y, 1, 0);
        visible += do_scan(&v, &mut vis, ym - 1, y, -1, 0);
    }

    Ok(visible)
}

fn do_scan(v: &Vec2D<u8>, vis: &mut Vec2D<bool>, mut x: i64, mut y: i64, dx: i64, dy: i64) -> u64 {
    let (xm, ym) = v.dims();
    let mut max_height_so_far = None;
    let mut visible = 0;
    while x >= 0 && y >= 0 && x < xm && y < ym {
        let this_height = v[(x, y)];
        if max_height_so_far.map_or(true, |h| h < this_height) {
            max_height_so_far = Some(this_height);
            if !vis[(x, y)] {
                visible += 1;
            }
            vis[(x, y)] = true;
        }
        x += dx;
        y += dy;
    }
    visible
}

pub fn solve_b() -> Result<i64> {
    let v = parse_input()?;

    let (xm, ym) = v.dims();

    let mut max_score = 0;
    for x in 1..xm - 1 {
        for y in 1..ym - 1 {
            max_score = max_score.max(scenic_score((x, y), &v));
        }
    }

    Ok(max_score)
}

// pub fn solve_b_opt() -> Result<u64> {
//     let v = parse_input()?;

//     let (xm, ym) = v.dims();

//     let mut maps:Vec<_> = (0..4).map(|_| Vec2D {
//         v: vec![-1i8; (xm * ym) as usize],
//         stride: xm,
//     }).collect();

//     for x in 0..xm {
//         do_scan_b(&v, &mut maps, x, 0, 0, 1);
//         do_scan_b(&v, &mut maps, x, xm-1, 0, -1);
//     }

//     for y in 0..ym {
//         do_scan_b(&v, &mut maps, 0, y, 1, 0);
//         do_scan_b(&v, &mut maps, ym-1, y, -1, 0);
//     }

//     for x in 0..xm {
//         for y in 0..ym {

//         }
//     }

//     Ok(visible)
// }

// fn do_scan_b(v: &Vec2D<u8>, maps: &mut Vec<Vec2D<i8>>, mut x: i64, mut y: i64, dx: i64, dy: i64) {
//     let (xm, ym) = v.dims();
//     let mut max_height_so_far = None;
//     let mut visible = 0;
//     while x >= 0 && y >= 0 && x < xm && y < ym {
//         let this_height = v[(x, y)];
//         if max_height_so_far.map_or(true, |h| h < this_height) {
//             max_height_so_far = Some(this_height);
//             if !vis[(x, y)] {
//                 visible += 1;
//             }
//             vis[(x, y)] = true;
//         }
//         x += dx;
//         y += dy;
//     }
//     visible
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 1688);
        assert_eq!(solve_a_opt().unwrap(), 1688);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 410400);
    }
}
