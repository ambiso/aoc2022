use std::io::BufRead;

use crate::error::Result;

fn parse_num(n: &str) -> i64 {
    n.chars().fold(0, |a, x| {
        let v = match x {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        };
        a * 5 + v
    })
}

fn fmt_base(mut n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut v = Vec::new();
    while n != 0 {
        v.push(n % 5);
        n /= 5;
    }

    let mut carry = 0;
    for i in 0..v.len() {
        v[i] += carry;
        carry = 0;
        if v[i] >= 3 {
            carry = (v[i] + 2) / 5;
            v[i] = -(5 - (v[i] % 5));
        }
    }
    if carry != 0 {
        v.push(carry);
    }

    v.iter()
        .rev()
        .map(|x| match *x {
            -5 => '0',
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        })
        .collect()
}

pub fn solve_a() -> Result<String> {
    let f = std::fs::read("inputs/day25a")?;
    let s = f
        .lines()
        .filter_map(|x| x.ok().map(|x| parse_num(x.as_str())))
        .sum();
    Ok(fmt_base(s))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_a() {
        // assert_eq!(solve_a().unwrap(), 3877);

        for x in 0..100000 {
            assert_eq!(x, parse_num(dbg!(fmt_base(dbg!(x))).as_str()));
        }
    }
    #[test]
    fn test_b() {
        // assert_eq!(solve_b().unwrap(), 982);
    }
}
