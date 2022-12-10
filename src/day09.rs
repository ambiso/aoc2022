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
struct State<const N: usize> {
    pos: [(i64, i64); N],
    visited: HashSet<(i64, i64)>,
}

fn solve<const N: usize>() -> Result<usize> {
    let f = std::fs::read("inputs/day09a")?;

    let (_, state) = parse_lines(
        &f,
        || State {
            pos: [(0, 0); N],
            visited: {
                let mut m = HashSet::new();
                m.insert((0, 0));
                m
            },
        },
        |mut state, line| {
            let dh = line.dir.into();

            for _ in 0..line.steps {
                state.pos[0] = addp(state.pos[0], dh);

                for i in 1..state.pos.len() {
                    let h_pos = state.pos[i - 1];
                    let t_pos = &mut state.pos[i];

                    loop {
                        let dx = h_pos.0 - t_pos.0;
                        let dy = h_pos.1 - t_pos.1;
                        if dx.abs() >= 2 || dy.abs() >= 2 {
                            t_pos.0 += dx.signum();
                            t_pos.1 += dy.signum();
                        } else {
                            break;
                        }
                    }
                }
                state.visited.insert(state.pos[state.pos.len() - 1]);
            }

            state
        },
    )
    .unwrap();

    Ok(state.visited.len())
}

pub fn solve_a() -> Result<usize> {
    solve::<2>()
}

pub fn solve_b() -> Result<usize> {
    solve::<10>()
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
        assert_eq!(solve_b().unwrap(), 2793);
    }
}
