use crate::{error::Result, util::read_string};

fn parse_input() -> Result<Vec<i64>> {
    let i = read_string("inputs/day20a")?;
    Ok(i.lines().map(|x| x.parse().unwrap()).collect())
}

fn solve(m: i64, n: i64) -> Result<i64> {
    let mut input = parse_input()?;
    input.iter_mut().for_each(|x| {
        *x *= m;
    });
    let mut mixed = Vec::from_iter(0..input.len());

    for _ in 0..n {
        for i in 0..input.len() {
            let p = mixed.iter().position(|x| *x == i).unwrap();
            let mut j = p as i64;
            let mut off = input[mixed[p]] % (input.len() - 1) as i64;

            let alt = off.signum() * (input.len() as i64 - off.abs() - 1);
            if off.abs() > alt.abs() {
                off = alt;
            }
            while off != 0 {
                let new_j = (j + off.signum()).rem_euclid(mixed.len() as i64);
                mixed.swap(j as usize, new_j as usize);
                j = new_j;
                off -= off.signum();
            }
        }
    }

    let idx = mixed.iter().position(|x| input[*x] == 0).unwrap();

    let s = [1000, 2000, 3000]
        .iter()
        .map(|&off| input[mixed[(idx + off) % mixed.len()]] as i64)
        .sum::<i64>();

    Ok(s)
}

pub fn solve_a() -> Result<i64> {
    solve(1, 1)
}
pub fn solve_b() -> Result<i64> {
    solve(811589153, 10)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 17490);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_a().unwrap(), 1632917375836);
    }
}
