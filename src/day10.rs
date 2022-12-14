use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{iterator, map, opt},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::error::Result;

fn parse_num(i: &[u8]) -> IResult<&[u8], i64> {
    map(
        |i| tuple((opt(tag(b"-")), digit1))(i),
        |(sign, x): (_, &[u8])| {
            let mut acc: i64 = 0;
            for d in x.iter() {
                acc *= 10;
                acc += (d - '0' as u8) as i64;
            }
            acc * if sign.is_some() { -1 } else { 1 }
        },
    )(i)
}

#[derive(Debug)]
struct Addx(i64);

fn parse_addx(i: &[u8]) -> IResult<&[u8], Addx> {
    map(preceded(tag(b"addx "), parse_num), Addx)(i)
}

#[derive(Debug)]
struct Noop;

fn parse_noop(i: &[u8]) -> IResult<&[u8], Noop> {
    map(tag(b"noop"), |_| Noop)(i)
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl From<Addx> for Instruction {
    fn from(value: Addx) -> Self {
        Instruction::Addx(value.0)
    }
}

impl From<Noop> for Instruction {
    fn from(_: Noop) -> Self {
        Instruction::Noop
    }
}

fn parse_instruction(i: &[u8]) -> IResult<&[u8], Instruction> {
    alt((map(parse_noop, Into::into), map(parse_addx, Into::into)))(i)
}

fn parse_line(i: &[u8]) -> IResult<&[u8], Instruction> {
    terminated(
        parse_instruction,
        opt(newline), // this is not really correct, it's only optional if it's the last instruction
    )(i)
}

#[derive(Debug, Clone, Copy)]
struct State {
    acc: i64,
}

enum TwoIterable<T> {
    NoElem,
    OneElem(T),
    TwoElems(T, T),
}

impl<T> Iterator for TwoIterable<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        use TwoIterable::*;
        let mut taken: TwoIterable<_> = NoElem;
        std::mem::swap(self, &mut taken);
        match taken {
            NoElem => None,
            OneElem(x) => Some(x),
            TwoElems(x, y) => {
                std::mem::swap(self, &mut OneElem(y));
                Some(x)
            }
        }
    }
}

pub fn solve_a() -> Result<i64> {
    let f = std::fs::read("inputs/day10a")?;

    let mut s = State { acc: 1 };
    let mut pit = iterator(&f[..], parse_line);
    let it = pit.flat_map(|ins| match ins {
        Instruction::Noop => TwoIterable::OneElem(s),
        Instruction::Addx(v) => {
            let prev_s = s.clone();
            s.acc += v;
            TwoIterable::TwoElems(prev_s, prev_s)
        }
    });

    let solution: i64 = it
        .enumerate()
        .filter_map(|(i, x)| {
            let i = i + 1;
            if i >= 20 && (i - 20) % 40 == 0 {
                Some(i as i64 * x.acc)
            } else {
                None
            }
        })
        .sum();

    // dbg!(pit.finish());

    // dbg!(&solution);

    Ok(solution)
}

// Works with fixed nom
// fn parse_input<'a: 'b, 'b>(i: &'a [u8]) -> impl Iterator<Item = State> + 'b {
//     let mut s = State { acc: 1 };
//     let mut pit = iterator(i, parse_line);
//     Iterator::flat_map(pit, move |ins| match ins {
//         Instruction::Noop => TwoIterable::OneElem(s),
//         Instruction::Addx(v) => {
//             let prev_s = s.clone();
//             s.acc += v;
//             TwoIterable::TwoElems(prev_s, prev_s)
//         }
//     })
// }

// use ouroboros::self_referencing;

// #[self_referencing]
// struct MyStruct<'a, A: 'a, F>
// where
//     for<'b> &'b mut A: Iterator<Item = Instruction>,
//     F: FnMut(Instruction) -> TwoIterable<State>,
// {
//     parent: A,
//     #[borrows(mut parent)]
//     #[not_covariant]
//     pub iter: std::iter::FlatMap<&'this mut A, TwoIterable<State>, F>,
//     phantom: std::marker::PhantomData<&'a ()>
// }

// impl<A, F> Iterator for MyStruct<'_, A, F>
// where
//     for<'a> &'a mut A: Iterator<Item = Instruction>,
//     F: FnMut(Instruction) -> TwoIterable<State>,
// {
//     type Item = State;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.with_iter_mut(|iter| iter.next())
//     }
// }

// #[allow(unused)]
// fn parse_input(
//     i: &[u8],
// ) -> MyStruct<
//     ParserIterator<
//         &[u8],
//         nom::error::Error<&[u8]>,
//         impl for<'a> Fn(&'a [u8]) -> IResult<&'a [u8], Instruction>,
//     >,
//     impl FnMut(Instruction) -> TwoIterable<State>,
// > {
//     let mut s = State { acc: 1 };
//     let pit = iterator(i, parse_line);
//     MyStructBuilder {
//         parent: pit,
//         iter_builder: |pit: &mut _| {
//             pit.flat_map(move |ins| match ins {
//                 Instruction::Noop => TwoIterable::OneElem(s),
//                 Instruction::Addx(v) => {
//                     let prev_s = s.clone();
//                     s.acc += v;
//                     TwoIterable::TwoElems(prev_s, prev_s)
//                 }
//             })
//         },
//         phantom: Default::default(),
//     }
//     .build()
// }

pub fn solve_b() -> Result<usize> {
    let f = std::fs::read("inputs/day10a")?;

    let mut s = State { acc: 1 };
    let mut pit = iterator(&f[..], parse_line);
    let it = pit.flat_map(|ins| match ins {
        Instruction::Noop => TwoIterable::OneElem(s),
        Instruction::Addx(v) => {
            let prev_s = s.clone();
            s.acc += v;
            TwoIterable::TwoElems(prev_s, prev_s)
        }
    });

    let mut crt = [false; 40 * 6];

    it.enumerate().for_each(|(i, s)| {
        let pos = i as i64 % 40;
        if s.acc == pos || s.acc - 1 == pos || s.acc + 1 == pos {
            crt[i] = true;
        }
    });

    // for y in 0..6 {
    //     for x in 0..40 {
    //         print!("{}", if crt[y * 40 + x] { '#' } else { '.' });
    //     }
    //     println!("");
    // }

    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 14220);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 0);
    }
}
