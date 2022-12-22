use std::collections::{HashMap, HashSet};

use crate::{error::Result, util::read_string};

type Name<'a> = &'a [u8];

#[derive(Debug, PartialEq, Eq)]
enum Op<'a> {
    Const(i64),
    Add(Name<'a>, Name<'a>),
    Mul(Name<'a>, Name<'a>),
    Sub(Name<'a>, Name<'a>),
    Div(Name<'a>, Name<'a>),
    Eq(Name<'a>, Name<'a>),
    Var,
}

fn eval_op<'a>(
    m: &HashMap<&'a [u8], Op<'a>>,
    cache: &mut HashMap<&'a [u8], Option<i64>>,
    name: &'a [u8],
) -> Option<i64> {
    if let Some(v) = cache.get(name) {
        *v
    } else {
        let op = &m[name];
        let v = match op {
            Op::Const(v) => Some(*v),
            Op::Add(a, b) => eval_op(m, cache, a).and_then(|x| eval_op(m, cache, b).map(|y| x + y)),
            Op::Mul(a, b) => eval_op(m, cache, a).and_then(|x| eval_op(m, cache, b).map(|y| x * y)),
            Op::Sub(a, b) => eval_op(m, cache, a).and_then(|x| eval_op(m, cache, b).map(|y| x - y)),
            Op::Div(a, b) => eval_op(m, cache, a).and_then(|x| eval_op(m, cache, b).map(|y| x / y)),
            Op::Eq(a, b) => {
                eval_op(m, cache, a).and_then(|x| eval_op(m, cache, b).map(|y| (x == y) as _))
            }
            Op::Var => None,
        };
        cache.insert(name, v);
        v
    }
}

pub fn solve_a() -> Result<i64> {
    let s = read_string("inputs/day21a")?;
    let mut map = HashMap::new();
    for l in s.lines() {
        let s = l.split(": ").collect::<Vec<_>>();
        let res = s[0].as_bytes();
        let inp = match s[1].split(" ").collect::<Vec<_>>().as_slice() {
            [v] => Op::Const(v.parse::<i64>().unwrap()),
            [a, "+", b] => Op::Add(a.as_bytes(), b.as_bytes()),
            [a, "-", b] => Op::Sub(a.as_bytes(), b.as_bytes()),
            [a, "*", b] => Op::Mul(a.as_bytes(), b.as_bytes()),
            [a, "/", b] => Op::Div(a.as_bytes(), b.as_bytes()),
            _ => panic!(),
        };

        map.insert(res, inp);
    }
    let mut cache = HashMap::new();
    let v = eval_op(&map, &mut cache, b"root").unwrap();
    Ok(v)
}

enum Direction {
    This,
    Left,
    Right,
}

fn find_var(map: &HashMap<&[u8], Op>, k: &[u8]) -> Option<Direction> {
    match map[k] {
        Op::Const(_) => None,
        Op::Add(a, b) | Op::Mul(a, b) | Op::Sub(a, b) | Op::Div(a, b) | Op::Eq(a, b) => {
            find_var(map, a)
                .map(|_| Direction::Left)
                .or_else(|| find_var(map, b).map(|_| Direction::Right))
        }
        Op::Var => Some(Direction::This),
    }
}

fn print_eq<'a>(visited: &mut HashSet<&'a [u8]>, map: &HashMap<&'a [u8], Op<'a>>, k: &'a [u8]) {
    if visited.contains(&k) {
        return;
    }
    visited.insert(k);
    let v = &map[k];
    let k = std::str::from_utf8(k).unwrap();
    match v {
        Op::Const(i) => println!("(define-fun {k} () Int {i})"),
        Op::Add(a, b) => {
            print_eq(visited, map, a);
            print_eq(visited, map, b);
            let a = std::str::from_utf8(a).unwrap();
            let b = std::str::from_utf8(b).unwrap();
            println!("(define-fun {k} () Int (+ {a} {b}))");
        }
        Op::Sub(a, b) => {
            print_eq(visited, map, a);
            print_eq(visited, map, b);
            let a = std::str::from_utf8(a).unwrap();
            let b = std::str::from_utf8(b).unwrap();
            println!("(define-fun {k} () Int (- {a} {b}))");
        }
        Op::Mul(a, b) => {
            print_eq(visited, map, a);
            print_eq(visited, map, b);
            let a = std::str::from_utf8(a).unwrap();
            let b = std::str::from_utf8(b).unwrap();
            println!("(define-fun {k} () Int (* {a} {b}))");
        }
        Op::Div(a, b) => {
            print_eq(visited, map, a);
            print_eq(visited, map, b);
            let a = std::str::from_utf8(a).unwrap();
            let b = std::str::from_utf8(b).unwrap();
            println!("(define-fun {k} () Int (div {a} {b}))");
        }
        Op::Eq(a, b) => {
            print_eq(visited, map, a);
            print_eq(visited, map, b);
            let a = std::str::from_utf8(a).unwrap();
            let b = std::str::from_utf8(b).unwrap();
            println!("(define-fun {k} () Bool (= {a} {b}))");
        }
        Op::Var => {
            println!("(declare-const {k} Int)");
        }
    }
}

pub fn solve_b() -> Result<i64> {
    let s = read_string("inputs/day21a")?;
    let mut map = HashMap::new();
    for l in s.lines() {
        let s = l.split(": ").collect::<Vec<_>>();
        let res = s[0].as_bytes();
        let inp = match (res, s[1].split(" ").collect::<Vec<_>>().as_slice()) {
            (b"root", [a, _, b]) => Op::Eq(a.as_bytes(), b.as_bytes()),
            (b"humn", _) => Op::Var,
            (_, [v]) => Op::Const(v.parse::<i64>().unwrap()),
            (_, [a, "+", b]) => Op::Add(a.as_bytes(), b.as_bytes()),
            (_, [a, "-", b]) => Op::Sub(a.as_bytes(), b.as_bytes()),
            (_, [a, "*", b]) => Op::Mul(a.as_bytes(), b.as_bytes()),
            (_, [a, "/", b]) => Op::Div(a.as_bytes(), b.as_bytes()),
            _ => panic!(),
        };

        map.insert(res, inp);
    }

    let names = map.keys().map(|x| Vec::from(*x)).collect::<Vec<_>>();
    let mut cache = HashMap::new();
    // constant propagation
    for k in names.iter() {
        if let Some(v) = eval_op(&map, &mut cache, &k[..]) {
            map.get_mut(&k[..]).map(|val| {
                *val = Op::Const(v);
            });
        }
    }
    // solve for var
    loop {
        let (left, right) = match map[b"root".as_slice()] {
            Op::Eq(l, r) => (l, r),
            _ => panic!(),
        };
        match find_var(&map, left) {
            Some(res) => match res {
                Direction::This => break,
                Direction::Left => match map[left] {
                    Op::Const(_) => todo!(),
                    Op::Add(a, b) => {
                        let mut cache = HashMap::new();
                        let v = eval_op(&map, &mut cache, right).unwrap()
                            - eval_op(&map, &mut cache, b).unwrap();
                        map.entry(right).and_modify(|x| *x = Op::Const(v));
                        match map.get_mut(b"root".as_slice()).unwrap() {
                            Op::Eq(l, _) => *l = a,
                            _ => todo!(),
                        }
                    }
                    Op::Mul(a, b) => {
                        let mut cache = HashMap::new();
                        let v = eval_op(&map, &mut cache, right).unwrap()
                            / eval_op(&map, &mut cache, b).unwrap();
                        map.entry(right).and_modify(|x| *x = Op::Const(v));
                        match map.get_mut(b"root".as_slice()).unwrap() {
                            Op::Eq(l, _) => *l = a,
                            _ => todo!(),
                        }
                    }
                    Op::Sub(a, b) => {
                        let mut cache = HashMap::new();
                        let v = eval_op(&map, &mut cache, right).unwrap()
                            + eval_op(&map, &mut cache, b).unwrap();
                        map.entry(right).and_modify(|x| *x = Op::Const(v));
                        match map.get_mut(b"root".as_slice()).unwrap() {
                            Op::Eq(l, _) => *l = a,
                            _ => todo!(),
                        }
                    }
                    Op::Div(a, b) => {
                        let mut cache = HashMap::new();
                        let v = eval_op(&map, &mut cache, right).unwrap()
                            * eval_op(&map, &mut cache, b).unwrap();
                        map.entry(right).and_modify(|x| *x = Op::Const(v));
                        match map.get_mut(b"root".as_slice()).unwrap() {
                            Op::Eq(l, _) => *l = a,
                            _ => todo!(),
                        }
                    }
                    Op::Eq(_, _) => todo!(),
                    Op::Var => todo!(),
                },
                Direction::Right => match map[left] {
                    Op::Const(_) => todo!(),
                    Op::Add(b, a) => {
                        let mut cache = HashMap::new();
                        let v = eval_op(&map, &mut cache, right).unwrap()
                            - eval_op(&map, &mut cache, b).unwrap();
                        map.entry(right).and_modify(|x| *x = Op::Const(v));
                        match map.get_mut(b"root".as_slice()).unwrap() {
                            Op::Eq(l, _) => *l = a,
                            _ => todo!(),
                        }
                    }
                    Op::Mul(b, a) => {
                        let mut cache = HashMap::new();
                        let v = eval_op(&map, &mut cache, right).unwrap()
                            / eval_op(&map, &mut cache, b).unwrap();
                        map.entry(right).and_modify(|x| *x = Op::Const(v));
                        match map.get_mut(b"root".as_slice()).unwrap() {
                            Op::Eq(l, _) => *l = a,
                            _ => todo!(),
                        }
                    }
                    Op::Sub(b, a) => {
                        let mut cache = HashMap::new();
                        let v = eval_op(&map, &mut cache, right).unwrap()
                            + eval_op(&map, &mut cache, b).unwrap();
                        map.entry(right).and_modify(|x| *x = Op::Const(v));
                        match map.get_mut(b"root".as_slice()).unwrap() {
                            Op::Eq(l, _) => *l = a,
                            _ => todo!(),
                        }
                    }
                    Op::Div(b, a) => {
                        let mut cache = HashMap::new();
                        let v = eval_op(&map, &mut cache, right).unwrap()
                            * eval_op(&map, &mut cache, b).unwrap();
                        map.entry(right).and_modify(|x| *x = Op::Const(v));
                        match map.get_mut(b"root".as_slice()).unwrap() {
                            Op::Eq(l, _) => *l = a,
                            _ => todo!(),
                        }
                    }
                    Op::Eq(_, _) => todo!(),
                    Op::Var => todo!(),
                },
            },
            _ => todo!(),
        }
    }

    // let mut visited = HashSet::new();
    // print_eq(&mut visited, &map, b"root");
    // println!("(assert (= root true))");
    // println!("(check-sat)");
    // println!("(get-model)");

    Ok(match map[b"root".as_slice()] {
        Op::Eq(_, right) => match map[right] {
            Op::Const(x) => x,
            _ => panic!(),
        },
        _ => panic!(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 72664227897438);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_a().unwrap(), 1632917375836);
    }
}
