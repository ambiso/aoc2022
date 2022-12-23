#![feature(portable_simd)]
#![feature(allocator_api)]
#![feature(ptr_as_uninit)]
#![feature(result_flattening)]
#![feature(byte_slice_trim_ascii)]
#![feature(iter_collect_into)]

use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod error;
mod util;

use std::hint::black_box;
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};
use util::format_duration;

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
    let solutions: Vec<Vec<BTreeMap<_, Box<dyn Fn() -> Box<dyn Debug>>>>> = vec![
        vec![dynfns!(day01::solve_a)],
        vec![
            dynfns!(day02::solve_a),
            dynfns!(day02::solve_b, day02::solve_b_opt, day02::solve_b_opt_2),
        ],
        vec![dynfns!(day03::solve_a), dynfns!(day03::solve_b)],
        vec![dynfns!(day04::solve_a)],
        vec![dynfns!(day05::solve_a), dynfns!(day05::solve_b)],
        vec![dynfns!(day06::solve_a), dynfns!(day06::solve_b)],
        vec![dynfns!(day07::solve_a), dynfns!(day07::solve_b)],
        vec![
            dynfns!(day08::solve_a, day08::solve_a_opt),
            dynfns!(day08::solve_b),
        ],
        vec![dynfns!(day09::solve_a), dynfns!(day09::solve_b)],
        vec![dynfns!(day10::solve_a), dynfns!(day10::solve_b)],
        vec![dynfns!(day11::solve_a), dynfns!(day11::solve_b)],
        vec![dynfns!(day12::solve_a), dynfns!(day12::solve_b)],
        vec![dynfns!(day13::solve_a), dynfns!(day13::solve_b)],
        vec![dynfns!(day14::solve_a), dynfns!(day14::solve_b)],
        vec![dynfns!(day15::solve_a), dynfns!(day15::solve_b)],
        vec![dynfns!(day16::solve_a), dynfns!(day16::solve_b)],
        vec![dynfns!(day17::solve_a), dynfns!(day17::solve_b)],
        vec![dynfns!(day18::solve_a), dynfns!(day18::solve_b)],
        vec![dynfns!(day19::solve_a), dynfns!(day19::solve_b)],
        vec![dynfns!(day20::solve_a), dynfns!(day20::solve_b)],
        vec![dynfns!(day21::solve_a), dynfns!(day21::solve_b)],
        vec![dynfns!(day22::solve_a), dynfns!(day22::solve_b)],
        vec![dynfns!(day23::solve_a), dynfns!(day23::solve_b)],
    ];

    let mut args = std::env::args();
    args.next().unwrap();
    let which = args.next().unwrap_or("1".to_string());

    if which == "bench" {
        let mut results = HashMap::new();
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
        let duration_per_test = Duration::from_millis(5000);
        let sample_chunk = 1;
        for i in which.iter() {
            let day = &solutions[*i];
            let day_no = i + 1;
            println!("Day {}", day_no);
            for (part_no, part) in day.iter().enumerate() {
                let part_no = part_no + 1;
                for (name, solution) in part {
                    let mut samples = 0;
                    let mut elapsed = Duration::ZERO;
                    let tic = Instant::now();
                    for _ in 0..sample_chunk {
                        black_box(solution());
                    }
                    let single_sample = tic.elapsed();
                    elapsed += single_sample;
                    samples += 1;

                    let sample_chunk = 1
                        .max((duration_per_test.as_nanos() / single_sample.as_nanos() / 10) as u32);
                    while elapsed < duration_per_test {
                        let tic = Instant::now();
                        for _ in 0..sample_chunk {
                            black_box(solution());
                        }
                        let chunk_elapsed = tic.elapsed();
                        elapsed += chunk_elapsed;
                        samples += sample_chunk;
                    }
                    let avg = elapsed / samples;
                    results
                        .entry((day_no, part_no))
                        .and_modify(|x| {
                            if *x > avg {
                                *x = avg;
                            }
                        })
                        .or_insert(avg);
                    total += avg;
                    println!(
                        "{name} computed in {} ({samples} samples)",
                        format_duration(avg)
                    );
                }
            }
            println!("");
        }

        println!("Total: {}", format_duration(total));
        println!("");
        println!("");
        println!("Day     Part 1      Part 2");

        let mut total_best = Duration::ZERO;
        for i in which.iter() {
            let day_no = i + 1;
            print!("{: >2}", day_no);
            for part_no in [1, 2] {
                match results.get(&(day_no, part_no)) {
                    Some(x) => {
                        total_best += *x;
                        print!("  {: >10}", format_duration(*x));
                    }
                    None => {
                        print!("         n/a");
                    }
                }
            }
            println!("");
        }
        println!("");
        println!("Total: {}", format_duration(total_best));
        return Ok(());
    }
    let which: usize = which.parse()?;
    let which_sub: usize = args.next().unwrap_or("1".to_string()).parse()?;

    let (_, f) = solutions[which - 1][which_sub - 1].iter().nth(0).unwrap();
    let tic = Instant::now();
    let res = f();
    let elapsed = tic.elapsed();
    println!("Result: {:?}", res);
    println!("Computed in {}", format_duration(elapsed));

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
