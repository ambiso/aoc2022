use std::{ops::Rem, collections::VecDeque};

use crate::{error::Result, util::read_string};
use rayon::prelude::*;

fn parse_input() -> Result<Vec<i32>> {
    let i = read_string("inputs/day20a")?;
    Ok(i.lines().map(|x| x.parse().unwrap()).collect())
}

fn mix_n(mix_fn: &[i32], tgt: Vec<i32>) -> Vec<i32> {
    let mut result = tgt.clone();
    result
        .par_iter_mut()
        .zip(mix_fn)
        .for_each(|(v, i)| *v = tgt[*i as usize]);
    result
}

pub fn solve_a() -> Result<i64> {
    let input = parse_input()?;
    // let new_pos = input.iter().enumerate().map(|(i, off)| (i as i32 + off).rem_euclid(input.len() as i32) as usize).collect::<Vec<_>>();
    // dbg!(&new_pos);
    // let mut sortperm = new_pos.iter().enumerate().map(|(i, new_pos)| (*new_pos as i32, i as i32)).collect::<Vec<_>>();
    // sortperm.sort();
    // let perm = sortperm.iter().map(|(_, original_pos)| *original_pos as i32).collect::<Vec<_>>();
    // dbg!(&input);
    // let output = mix_n(&perm[..], input);
    // dbg!(perm);
    // dbg!(output);

    // let mut perm: Vec<_> = input.clone();
    // let mut perm: Vec<_> = (0..input.len()).collect();

    // for (i, off)  in input.iter().enumerate() {
    //     let mut j = i as i64;
    //     let mut off = *off;
    //     while off != 0 {
    //         let j_new = (j + off.signum() as i64).rem_euclid(perm.len() as i64);
    //         perm.swap(j as usize, j_new as usize);
    //         off += -off.signum();
    //         j = j_new;
    //     }
    //     if i == 1 {
    //         break;
    //     }
    // }

    let mut mixed = VecDeque::from(input.clone());

    for v in input.iter() {
        let p = mixed.iter().position(|x| *x == *v).unwrap();
        let mut j = p as i64;
        let mut off = *v as i64;

        while off != 0 {
            let new_j = (j + off.signum()).rem_euclid(mixed.len() as i64);
            mixed.swap(j as usize, new_j as usize);
            j = new_j;
            off -= off.signum();
        }
    }

    let idx = mixed.iter().position(|x| *x == 0).unwrap();

    let f = |off: usize| {
        mixed[(idx + off) % mixed.len()]
    };

    let s = [1000, 2000, 3000].iter().map(|off| f(*off)).sum::<i32>();

    Ok(s as i64)
}
pub fn solve_b() -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 1382);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_a().unwrap(), 31740);
    }
}
