use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{newline, space0, space1},
    combinator::{eof, iterator, map, all_consuming, opt},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Finish,
};

use crate::{error::Result, util::{parse_num, gcd}};

#[derive(Debug)]
enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

#[derive(Debug)]
enum Operand {
    Old,
    Const(i64),
}

impl Operand {
    fn eval(&self, old: i64) -> i64 {
        match self {
            Operand::Old => old,
            Operand::Const(x) => *x,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: i64,
    action_true: i64,
    action_false: i64,
}

impl Operation {
    fn eval(&self, old: i64) -> i64 {
        match self {
            Operation::Add(x, y) => x.eval(old) + y.eval(old),
            Operation::Multiply(x, y) => x.eval(old) * y.eval(old),
        }
    }
}

fn item_list(i: &[u8]) -> IResult<&[u8], VecDeque<i64>> {
    map(separated_list0(tag(b", "), parse_num), Into::into)(i)
}

fn parse_operand(i: &[u8]) -> IResult<&[u8], Operand> {
    alt((
        map(tag(b"old"), |_| Operand::Old),
        map(parse_num, Operand::Const),
    ))(i)
}

fn parse_operation(i: &[u8]) -> IResult<&[u8], Operation> {
    map(
        tuple((
            parse_operand,
            alt((tag(b" * "), tag(b" + "))),
            parse_operand,
        )),
        |(l, op, r)| match op {
            b" * " => Operation::Multiply(l, r),
            b" + " => Operation::Add(l, r),
            _ => unreachable!(),
        },
    )(i)
}

fn parse_monkey(i: &[u8]) -> IResult<&[u8], Monkey> {
    map(
        tuple((
            delimited(tag(b"Monkey "), parse_num, tag(b":\n")),
            delimited(
                tuple((space1, tag(b"Starting items: "))),
                item_list,
                newline,
            ),
            delimited(
                tuple((space1, tag(b"Operation: new = "))),
                parse_operation,
                newline,
            ),
            delimited(
                tuple((space1, tag(b"Test: divisible by "))),
                parse_num,
                newline,
            ),
            delimited(
                tuple((space1, tag(b"If true: throw to monkey "))),
                parse_num,
                newline,
            ),
            preceded(
                tuple((space1, tag(b"If false: throw to monkey "))),
                parse_num,
            ),
        )),
        |(_monkey_id, items, operation, test, action_true, action_false)| {
            Monkey {
                items,
                operation,
                test,
                action_true,
                action_false
            }
        },
    )(i)
}

fn parse_monkeys(i: &[u8]) -> IResult<&[u8], Vec<Monkey>> {
    all_consuming(separated_list0(
        take_while1(|x: u8| x.is_ascii_whitespace()),
        terminated(parse_monkey, opt(newline)),
    ))(i)
}

pub fn solve(n: i64, div_three: bool) -> Result<i64> {
    let f = std::fs::read("inputs/day11a")?;

    let mut monkeys = parse_monkeys(&f[..]).finish().unwrap().1;
    let mut inspections = vec![0; monkeys.len()];
    let modulus: i64 = monkeys.iter().map(|x| x.test).product::<i64>();

    for _ in 0..n {
        // print_monkeys(&monkeys);
        // println!("");
        sim_round(&mut monkeys, &mut inspections, modulus, div_three);
        // if [1, 20, 1000].contains(&(i+1)) {
        //     println!("== After round {} ==", i+1);
        //     for (i, ins) in inspections.iter().enumerate() {
        //         println!("Monkey {i} inspected items {ins} times.");
        //     }
        //     println!("");
        // }
    }
    inspections.sort();
    // print_monkeys(&monkeys);

    Ok(inspections[inspections.len()-1] * inspections[inspections.len() - 2])
}

pub fn solve_a() -> Result<i64> {
    solve(20, true)
}

#[allow(unused)]
fn print_monkeys(monkeys: &[Monkey]) {
    for (i, m) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", i, m.items);
    }
}

fn sim_round(monkeys: &mut [Monkey], inspections: &mut [i64], modulus: i64, div_three: bool) {
    for m in 0..monkeys.len() {
        loop {
            let monkey = &mut monkeys[m];
            let mut wl = match monkey.items.pop_front() {
                Some(x) => x,
                None => break
            };
            inspections[m] += 1;
            wl = monkey.operation.eval(wl);
            if div_three {
                wl /= 3;
            }
            wl %= modulus;

            let next = if wl % monkey.test == 0 {
                monkey.action_true
            } else {
                monkey.action_false
            };

            monkeys[next as usize].items.push_back(wl);
        }
    }
}

pub fn solve_b() -> Result<i64> {
    solve(10000, false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 121450);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 28244037010);
    }
}
