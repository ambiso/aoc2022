#![feature(portable_simd)]
#![feature(allocator_api)]
#![feature(ptr_as_uninit)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod error;
mod util;

use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::error::Result;

macro_rules! dynfn {
    ($x:expr) => {
        Box::new(|| Box::new($x()))
    };
}

macro_rules! dynfns {
    ($x:expr) => {
        vec![dynfn!($x)]
    };
    ($x:expr, $($y:expr),+) => {
        vec![dynfn!($x),$(dynfn!($y)),+]
    };
}

fn main() -> Result<()> {
    let solutions: Vec<Vec<Box<dyn Fn() -> Box<dyn Debug>>>> = vec![
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
        dynfns!(day07::solve_a),
    ];

    let mut args = std::env::args();
    args.next().unwrap();
    let which = args.next().unwrap_or("1".to_string());
    if which == "all" {
        let mut total = Duration::ZERO;
        let n = 10000;
        for j in 0..n {
            for (i, day) in solutions.iter().enumerate() {
                if j == 0 {
                println!("Day {}", i + 1);
                }
                for solution in day {
                    let tic = Instant::now();
                    solution();
                    let elapsed = tic.elapsed();
                    total += elapsed;
                    if j == 0 {
                    println!("Computed in {}µs", elapsed.as_nanos() as f64 / 1000.0);
                    }
                }
                if j == 0 {
                println!("");
                }
            }
        }

        println!("Total: {}µs", total.as_nanos() as f64 / 1000.0 / n as f64);
        return Ok(());
    }
    let which: usize = which.parse()?;
    let which_sub: usize = args.next().unwrap_or("0".to_string()).parse()?;

    let tic = Instant::now();
    let res = solutions[which - 1][which_sub]();
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
