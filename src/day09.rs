use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, opt},
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::error::Result;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&[u8]> for Direction {
    fn from(value: &[u8]) -> Self {
        use Direction::*;
        match value[0] as char {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!(),
        }
    }
}

impl From<Direction> for (i64, i64) {
    fn from(value: Direction) -> Self {
        use Direction::*;
        match value {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

fn addp(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn parse_dir(i: &[u8]) -> IResult<&[u8], Direction> {
    map(
        alt((
            tag(&['U' as u8]),
            tag(&['D' as u8]),
            tag(&['L' as u8]),
            tag(&['R' as u8]),
        )),
        Into::into,
    )(i)
}

fn parse_num(i: &[u8]) -> IResult<&[u8], u8> {
    map(digit1, |x: &[u8]| {
        let mut acc = 0;
        for d in x.iter() {
            acc *= 10;
            acc += d - '0' as u8;
        }
        acc
    })(i)
}

struct Line {
    dir: Direction,
    steps: u8,
}

fn parse_line(i: &[u8]) -> IResult<&[u8], Line> {
    map(
        terminated(
            separated_pair(parse_dir, tag(&[' ' as u8]), parse_num),
            opt(newline),
        ),
        |(dir, steps)| Line { dir, steps },
    )(i)
}

fn parse_lines<T>(
    i: &[u8],
    init: impl FnMut() -> T,
    f: impl FnMut(T, Line) -> T,
) -> IResult<&[u8], T> {
    fold_many1(parse_line, init, f)(i)
}

#[derive(Debug)]
struct State {
    h_pos: (i64, i64),
    t_pos: (i64, i64),
    visited: HashSet<(i64, i64)>,
}

pub fn solve_a() -> Result<usize> {
    let f = std::fs::read("inputs/day09a")?;

    let (_, state) = parse_lines(
        &f,
        || State {
            h_pos: (0, 0),
            t_pos: (0, 0),
            visited: HashSet::new(),
        },
        |mut state, line| {
            state.visited.insert(state.t_pos);
            let dh = line.dir.into();

            for _ in 0..line.steps {
                state.h_pos = addp(state.h_pos, dh);

                let mut dx = state.h_pos.0 - state.t_pos.0;
                if dx.abs() == 2 && state.h_pos.1 == state.t_pos.1 {
                    state.t_pos.0 += 1 * dx.signum();
                }

                let mut dy = state.h_pos.1 - state.t_pos.1;
                if dy.abs() == 2 && state.h_pos.0 == state.t_pos.0 {
                    state.t_pos.1 += 1 * dy.signum();
                }

                if dx.abs() + dy.abs() == 3 {
                    if dx.abs() == 2 {
                        dx /= 2;
                    }
                    if dy.abs() == 2 {
                        dy /= 2;
                    }

                    state.t_pos = addp(state.t_pos, (dx, dy));
                }
                state.visited.insert(state.t_pos);
            }

            state
        },
    )
    .unwrap();

    // dbg!(&state);

    Ok(state.visited.len())
}

pub fn solve_b() -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 6745);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 410400);
    }
}
