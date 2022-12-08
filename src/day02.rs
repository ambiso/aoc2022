use crate::error::Result;
use crate::util::read_string;
use rand::distributions::Uniform;
use rand::Rng;
use std::arch::x86_64::{__m256i, _mm256_shuffle_epi8};
use std::fs;
use std::simd::{u8x32, SimdUint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn convert_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

fn convert_mine(c: char) -> RPS {
    match c {
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => panic!(),
    }
}

fn convert_other(c: char) -> RPS {
    match c {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!(),
    }
}

fn what_score(mine: RPS) -> i64 {
    match mine {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn dominates(x: RPS) -> RPS {
    match x {
        RPS::Rock => RPS::Scissors,
        RPS::Paper => RPS::Rock,
        RPS::Scissors => RPS::Paper,
    }
}

fn dominated_by(x: RPS) -> RPS {
    match x {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissors,
        RPS::Scissors => RPS::Rock,
    }
}

pub fn solve_b() -> Result<i64> {
    let mut score = 0;
    for l in read_string("inputs/day02g")?.lines() {
        let mut s = l.chars();
        let other = convert_other(s.next().unwrap());
        s.next();
        let desired_outcome = convert_outcome(s.next().unwrap());
        let mine = match desired_outcome {
            Outcome::Win => dominated_by(other),
            Outcome::Lose => dominates(other),
            Outcome::Draw => other,
        };

        let outcome = if mine == other {
            Outcome::Draw
        } else if dominates(other) == mine {
            Outcome::Lose
        } else if dominates(mine) == other {
            Outcome::Win
        } else {
            unreachable!()
        };
        let s = match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        } + what_score(mine);
        score += s;
    }
    Ok(score)
}

#[allow(unused)]
pub fn gen_lut() {
    for other_char in ['A', 'B', 'C'] {
        for outcome_char in ['X', 'Y', 'Z'] {
            let other = convert_other(other_char);
            let desired_outcome = convert_outcome(outcome_char);
            let mine = match desired_outcome {
                Outcome::Win => dominated_by(other),
                Outcome::Lose => dominates(other),
                Outcome::Draw => other,
            };

            let outcome = if mine == other {
                Outcome::Draw
            } else if dominates(other) == mine {
                Outcome::Lose
            } else if dominates(mine) == other {
                Outcome::Win
            } else {
                unreachable!()
            };
            println!(
                "{other_char}{outcome_char}{}",
                match outcome {
                    Outcome::Win => 6,
                    Outcome::Draw => 3,
                    Outcome::Lose => 0,
                } + what_score(mine)
            );
        }
    }
}

pub fn solve_a() -> Result<i64> {
    let mut score = 0;
    for l in read_string("inputs/day02a")?.lines() {
        let mut s = l.chars();
        let other = convert_other(s.next().unwrap());
        s.next();
        let mine = convert_mine(s.next().unwrap());

        let outcome = if mine == other {
            Outcome::Draw
        } else if dominates(other) == mine {
            Outcome::Lose
        } else if dominates(mine) == other {
            Outcome::Win
        } else {
            panic!()
        };
        score += match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        } + what_score(mine);
    }
    Ok(score)
}

#[allow(unused)]
pub fn gen_input_day2() -> Result<()> {
    let mut s = Vec::<u8>::new();

    let mut rng = rand::thread_rng();
    for _ in 0..100_000_000 {
        let idx = rng.sample(Uniform::new(0, 3));
        s.push(['A' as u8, 'B' as u8, 'C' as u8][idx]);
        s.push(' ' as u8);
        s.push(['X' as u8, 'Y' as u8, 'Z' as u8][idx]);
        s.push('\n' as u8);
    }
    fs::write("day02g", s)?;
    Ok(())
}

// fn scorep1(byte: u8) -> u8 {
//     let isdraw = (byte == 0x33) | (byte == 0x22) | (byte == 0x11);
//     let iswin = (byte == 0x12) | (byte == 0x23) | (byte == 0x31);
//     (byte & 0x0f) + (0x3 * isdraw as u8) + (0x6 * iswin as u8)
// }

pub fn solve_b_opt() -> Result<i64> {
    let mut score = 0i64;
    let v = fs::read("inputs/day02a")?;
    let lut = [
        2, 3, 7, 0, 4, 8, 1, 5, 6, 0, 0, 0, 0, 0, 0, 0, 2, 3, 7, 0, 4, 8, 1, 5, 6, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let simd_lut = __m256i::from(u8x32::from(lut));
    let m_d1 = u8x32::from([
        0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00,
        0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00,
        0x00, 0x00,
    ]);
    let m_d2 = m_d1.rotate_lanes_right::<2>();
    let mut chunks = v.chunks_exact(32 * 4);
    for l in &mut chunks {
        let l1 = u8x32::from_slice(&l[32 * 0..32 * 1]);
        let l2 = u8x32::from_slice(&l[32 * 1..32 * 2]);
        let l3 = u8x32::from_slice(&l[32 * 2..32 * 3]);
        let l4 = u8x32::from_slice(&l[32 * 3..32 * 4]);

        let l1_d1 = l1 & m_d1;
        let l2_d1 = l2 & m_d1;
        let l3_d1 = l3 & m_d1;
        let l4_d1 = l4 & m_d1;

        let l1_d2 = l1 & m_d2;
        let l2_d2 = l2 & m_d2;
        let l3_d2 = l3 & m_d2;
        let l4_d2 = l4 & m_d2;

        let d1s = l1_d1
            | l2_d1.rotate_lanes_right::<1>()
            | l3_d1.rotate_lanes_right::<2>()
            | l4_d1.rotate_lanes_right::<3>();
        let d2s = l1_d2.rotate_lanes_left::<2>()
            | l2_d2.rotate_lanes_left::<1>()
            | l3_d2
            | l4_d2.rotate_lanes_right::<1>();
        let vals = (u8x32::from(d1s) - u8x32::splat('A' as u8)) * u8x32::splat(3)
            + (u8x32::from(d2s) - u8x32::splat('X' as u8));
        let vals = __m256i::from(vals);
        let scores = unsafe { _mm256_shuffle_epi8(simd_lut, vals) };
        score += u8x32::from(scores).reduce_sum() as i64 + 32;
    }
    let rem = chunks.remainder();
    for l in 0..rem.len() / 4 {
        let d1 = rem[l * 4] - 'A' as u8;
        let d2 = rem[l * 4 + 2] - 'X' as u8;
        score += lut[(d1 * 3 + d2) as usize] as i64 + 1;
    }
    Ok(score)
}

pub fn solve_b_opt_2() -> Result<i64> {
    let mut score = 0i64;
    let v = fs::read("inputs/day02a")?;
    let lut = [2, 3, 7, 0, 4, 8, 1, 5, 6];
    for l in 0..v.len() / 4 {
        let d1 = v[l * 4] - 'A' as u8;
        let d2 = v[l * 4 + 2] - 'X' as u8;
        score += lut[(d1 * 3 + d2) as usize] as i64 + 1;
    }
    Ok(score)
}
