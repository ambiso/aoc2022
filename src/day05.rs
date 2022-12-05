use crate::{error::Result, util::read_string};

// use nom::{
//     bytes::complete::tag,
//     character::complete::{alpha1, char},
//     multi::many1,
//     sequence::{delimited, preceded, terminated},
//     IResult,
// };

// fn parse_input(input: &str) -> IResult<&str, Vec<Vec<(char, u32)>>> {
//     many1(parse_row)(input)
// }

// fn parse_row(input: &str) -> IResult<&str, Vec<(char, u32)>> {
//     many1(parse_cell)(input)
// }

// fn parse_cell(input: &str) -> IResult<&str, (char, u32)> {
//     let (input, _) = char('[')(input)?;
//     let (input, letter) = alpha1(input)?;
//     let (input, _) = char(']')(input)?;
//     let (input, number) = terminated(preceded(char(' '), nom::character::complete::digit1), char(' '))(input)?;
//     Ok((input, (letter, number.parse().unwrap())))
// }


pub fn solve_a() -> Result<String>{
    
    let mut stacks = Vec::new();

    let s = read_string("inputs/day05a")?;
    let mut lines = s.lines();

    for l in &mut lines {
        if stacks.len() == 0 {
            for _ in 0..l.len() / 4 + 1 {
                stacks.push(Vec::new());
            }
        }

        if l.as_bytes()[1] == '1' as u8 {
            break;
        }

        for i in 0..stacks.len() {
            let c = l.as_bytes()[i * 4 + 1];
            if c != ' ' as u8 {
                stacks[i].push(c);
            }
        }
    }

    for s in stacks.iter_mut() {
        s.reverse();
    }

    lines.next().unwrap();
    for l in &mut lines {
        let s = l.split(' ').collect::<Vec<_>>();
        let n: u64 = s[1].parse()?;
        let from: usize = s[3].parse()?;
        let to: usize = s[5].parse()?;

        for _ in 0..n {
            let lift = stacks[from-1].pop().unwrap();
            stacks[to-1].push(lift);
        }
    }
    Ok(stacks.iter().map(|x| x.last().unwrap_or(&(' ' as u8))).fold("".to_string(), |mut a, x| {
        a.push(*x as char);
        a
    }))
}

pub fn solve_b() -> Result<String>{
    
    let mut stacks = Vec::new();

    let s = read_string("inputs/day05a")?;
    let mut lines = s.lines();

    for l in &mut lines {
        if stacks.len() == 0 {
            for _ in 0..l.len() / 4 + 1 {
                stacks.push(Vec::new());
            }
        }

        if l.as_bytes()[1] == '1' as u8 {
            break;
        }

        for i in 0..stacks.len() {
            let c = l.as_bytes()[i * 4 + 1];
            if c != ' ' as u8 {
                stacks[i].push(c);
            }
        }
    }

    for s in stacks.iter_mut() {
        s.reverse();
    }

    lines.next().unwrap();
    for l in &mut lines {
        let s = l.split(' ').collect::<Vec<_>>();
        let n: usize = s[1].parse()?;
        let from: usize = s[3].parse()?;
        let to: usize = s[5].parse()?;

        let fsl = stacks[from-1].len();
        let chunk = Vec::from(&stacks[from-1][fsl-n..]);
        stacks[to-1].extend(chunk);
        stacks[from-1].truncate(fsl-n);
    }
    Ok(stacks.iter().map(|x| x.last().unwrap_or(&(' ' as u8))).fold("".to_string(), |mut a, x| {
        a.push(*x as char);
        a
    }))
}