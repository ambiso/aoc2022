use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many0, IResult};

use crate::{error::Result, util::parse_num};

#[derive(Debug)]
enum Instruction {
    Step(i64),
    Left,
    Right,
}

fn parse_instructions(i: &[u8]) -> IResult<&[u8], Vec<Instruction>> {
    many0(alt((
        map(parse_num, Instruction::Step),
        map(tag(b"L"), |_| Instruction::Left),
        map(tag(b"R"), |_| Instruction::Right),
    )))(i)
}

fn get_pos(map: &[&[u8]], p: [i32; 2]) -> Option<u8> {
    p[1].try_into()
        .ok()
        .and_then(|y: usize| map.get(y))
        .zip(TryInto::<usize>::try_into(p[0]).ok())
        .and_then(|(xs, x)| xs.get(x).map(|x| *x))
}

pub fn solve_a() -> Result<i64> {
    let s = std::fs::read("inputs/day22a")?;
    let mut map = Vec::new();

    let mut lines = s.split(|x| *x == '\n' as u8);
    for l in &mut lines {
        if l.len() == 0 {
            break;
        }
        map.push(l);
    }
    let mut p = [
        map[0].iter().position(|x| *x == '.' as u8).unwrap() as i32,
        0,
    ];
    let mut dir = 0i32;
    let dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]];
    let instructions = parse_instructions(lines.next().unwrap()).unwrap().1;

    for i in instructions.iter() {
        match i {
            Instruction::Step(n) => {
                let d = dirs[dir as usize];
                for j in 0..*n {
                    let new_p = [
                        (p[0] + d[0]).rem_euclid(map[0].len() as i32),
                        (p[1] + d[1]).rem_euclid(map.len() as i32),
                    ];
                    let b = get_pos(&map, new_p);
                    if b.unwrap_or(' ' as u8) == '.' as u8 {
                        p = new_p;
                    } else if b.unwrap_or(' ' as u8) == ' ' as u8 {
                        let mut new_p = new_p;
                        while ' ' as u8 == get_pos(&map, new_p).unwrap_or(' ' as u8) {
                            new_p = [
                                (new_p[0] + d[0]).rem_euclid(map[0].len() as i32),
                                (new_p[1] + d[1]).rem_euclid(map.len() as i32),
                            ];
                        }
                        let v = get_pos(&map, new_p).unwrap();
                        if v == '.' as u8 {
                            p = new_p;
                        } else if v == '#' as u8 {
                            break;
                        }
                    }
                }
            }
            Instruction::Left => {
                dir -= 1;
                dir = dir.rem_euclid(dirs.len() as i32);
            }
            Instruction::Right => {
                dir += 1;
                dir = dir.rem_euclid(dirs.len() as i32);
            }
        }
    }

    Ok((1000 * (p[1] + 1) + 4 * (p[0] + 1) + dir) as i64)
}

pub fn solve_b() -> Result<i64> {
    Ok(0)
}
