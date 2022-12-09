#![feature(portable_simd)]
#![feature(allocator_api)]
#![feature(ptr_as_uninit)]

use std::collections::btree_map::BTreeMap;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod error;
mod util;

use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::error::Result;

macro_rules! dynfn {
    ($x:expr) => {
        Box::new(|| Box::new($x()) as Box<dyn Debug>) as Box<dyn Fn() -> Box<dyn Debug>>
    };
}

macro_rules! dynfns_impl {
    ($m:expr, $x:expr) => {
        $m.insert(std::stringify!($x), dynfn!($x));
    };
    ($m:expr, $x:expr, $($y:expr),+) => {
        {
            $m.insert(std::stringify!($x), dynfn!($x));
            dynfns_impl!($m, $($y),+);
        }
    };
}

macro_rules! dynfns {
    ($x:expr) => {
        {
            let mut m = BTreeMap::new();
            m.insert(std::stringify!($x), dynfn!($x));
            m
        }
    };
    ($x:expr, $($y:expr),+) => {
        {
        let mut m = BTreeMap::new();
        dynfns_impl!(m, $x, $($y),+);
        m
        }
    };
}

fn main() -> Result<()> {
    let solutions: Vec<BTreeMap<_, Box<dyn Fn() -> Box<dyn Debug>>>> = vec![
        dynfns!(day01::solve_a),
        dynfns!(
            day02::solve_a,
            day02::solve_b,
            day02::solve_b_opt,
            day02::solve_b_opt_2
        ),
        dynfns!(day03::solve_a, day03::solve_b),
        dynfns!(day04::solve_a),
        dynfns!(day05::solve_a, day05::solve_b),
        dynfns!(day06::solve_a, day06::solve_b),
        dynfns!(day07::solve_a, day07::solve_b),
        dynfns!(day08::solve_a, day08::solve_b, day08::solve_a_opt),
        dynfns!(day09::solve_a, day09::solve_b),
    ];

    let mut args = std::env::args();
    args.next().unwrap();
    let which = args.next().unwrap_or("1".to_string());
    if which == "bench" {
        let which = args.next().unwrap_or("all".to_string());
        let which = which.split(",").collect::<Vec<_>>();
        let which: Vec<_> = if which[0] == "all" {
            (0..solutions.len()).collect()
        } else {
            which
                .iter()
                .filter_map(|x| x.parse::<usize>().ok().map(|x| x - 1))
                .collect()
        };
        let mut total = Duration::ZERO;
        let n = 10000;
        println!("Samples: {n}");
        for i in which {
            let day = &solutions[i];
            println!("Day {}", i + 1);
            for (name, solution) in day {
                let tic = Instant::now();
                for _ in 0..n {
                    solution();
                }
                let elapsed = tic.elapsed();
                total += elapsed;
                println!(
                    "{name} computed in {}µs",
                    elapsed.as_nanos() as f64 / 1000.0 / n as f64
                );
            }
            println!("");
        }

        println!("Total: {}µs", total.as_nanos() as f64 / 1000.0 / n as f64);
        return Ok(());
    }
    let which: usize = which.parse()?;
    let which_sub: usize = args.next().unwrap_or("0".to_string()).parse()?;

    let (_, f) = solutions[which - 1].iter().nth(which_sub).unwrap();
    let tic = Instant::now();
    let res = f();
    let elapsed = tic.elapsed();
    println!("Result: {:?}", res);
    println!("Computed in {}µs", elapsed.as_nanos() as f64 / 1000.0);

    // day01::solve_a()?;
    // day02::solve_a()?;
    // day02::solve_b_opt()?;
    // gen_lut();
    // day02::solve_b()?;
    // dbg!(day03::solve_a()?);
    // dbg!(day03::solve_b()?);
    // dbg!(day04::solve_a()?);
    // dbg!(day05::solve_b()?);
    // assert_eq!(orig, opt);
    // gen_input_day2()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() -> Result<()> {
        day01::solve_a()?;
        day02::solve_a()?;
        // day02::solve_b_opt()?;
        day02::gen_lut();
        day02::solve_b()?;
        dbg!(day03::solve_a()?);
        dbg!(day03::solve_b()?);
        dbg!(day04::solve_a()?);
        dbg!(day05::solve_b()?);
        // gen_input_day2()?;
        Ok(())
    }
}
