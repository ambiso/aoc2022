use std::collections::HashMap;

use crate::{error::Result, util::read_string};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

// ore
// clay
// obsidian
// geode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Materials([i64; 4]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cost([i64; 4]);

// ore robot
// clay robot
// obsidian robot
// geode robot
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Machines([i64; 4]);

lazy_static! {
    static ref RE: Regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

// Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 13 clay. Each geode robot costs 3 ore and 7 obsidian.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Blueprint([Cost; 4]);

fn parse_input() -> Result<Vec<Blueprint>> {
    let i = read_string("inputs/day19a")?;
    Ok(RE
        .captures_iter(i.as_str())
        .map(|c| {
            let mut c = c
                .iter()
                .skip(1)
                .flatten()
                .filter_map(|x| x.as_str().parse::<i64>().ok());
            Blueprint([
                Cost([c.next().unwrap(), 0, 0, 0]),
                Cost([c.next().unwrap(), 0, 0, 0]),
                Cost([c.next().unwrap(), c.next().unwrap(), 0, 0]),
                Cost([c.next().unwrap(), 0, c.next().unwrap(), 0]),
            ])
        })
        .collect())
}

fn solve_bp(bp: &Blueprint, time_left: i64) -> i64 {
    let mut cache = HashMap::new();
    solve_internal(
        bp,
        &mut cache,
        time_left,
        Materials([0, 0, 0, 0]),
        Machines([1, 0, 0, 0]),
    )
}

fn step(bp: &Blueprint, time_left: i64, mut materials: Materials, machines: Machines) -> Materials {
    materials
        .0
        .iter_mut()
        .zip(machines.0)
        .enumerate()
        .for_each(|(mat_id, (mat, mac))| {
            *mat += mac;
            if mat_id != 3 {
                // it never makes sense to collect more of a material than you can spend
                *mat = (*mat).min(time_left * bp.0.iter().map(|c| c.0[mat_id]).max().unwrap());
            }
        });
    materials
}

fn value(materials: Materials) -> i64 {
    materials.0[3]
}

fn buy(
    bp: &Blueprint,
    machine_cost: Cost,
    mut materials: Materials,
    mut machines: Machines,
    machine_id: usize,
) -> Option<(Materials, Machines)> {
    if materials
        .0
        .iter()
        .zip(machine_cost.0.iter())
        .all(|(material, requirement)| material >= requirement)
    {
        materials
            .0
            .iter_mut()
            .zip(machine_cost.0)
            .for_each(|(mat, cost)| *mat -= cost);
        let mc = &mut machines.0[machine_id];
        *mc += 1;
        // it never makes sense to have more machines than required to buy a new machine per tick
        if machine_id != 3 {
            let max_material_cost = bp.0.iter().map(|c| c.0[machine_id]).max().unwrap();
            *mc = (*mc).min(max_material_cost);
        }
        Some((materials, machines))
    } else {
        None
    }
}

fn compress_state(time_left: i64, materials: Materials, machines: Machines) -> i128 {
    let mut i = 0i128;
    let m = 256;
    i += time_left as i128;
    for x in materials.0 {
        i *= m;
        assert!(x < m as i64, "{:?} {:?}", materials, machines);
        i += x as i128;
    }
    for x in machines.0 {
        i *= m;
        assert!(x < m as i64, "{:?} {:?}", machines, machines);
        i += x as i128;
    }
    i
}

fn solve_internal(
    bp: &Blueprint,
    cache: &mut HashMap<i128, i64>,
    time_left: i64,
    materials: Materials,
    machines: Machines,
) -> i64 {
    if time_left == 0 {
        return value(materials);
    }

    if let Some(x) = cache.get(&compress_state(time_left, materials, machines)) {
        *x
    } else {
        let mut best = 0;

        let mut can_buy = [false; 4];
        // buy machine of each type
        for (machine_id, machine_cost) in bp.0.iter().enumerate() {
            if let Some((new_materials, new_machines)) =
                buy(bp, *machine_cost, materials, machines, machine_id)
            {
                can_buy[machine_id] = true;
                best = best.max(solve_internal(
                    bp,
                    cache,
                    time_left - 1,
                    step(bp, time_left, new_materials, machines),
                    new_machines,
                ))
            }
        }
        if bp
            .0
            .iter()
            .zip(can_buy.iter())
            .any(|(cost, can_buy)| !can_buy && will_be_buyable(*cost, machines))
        {
            // if we cannot buy all machines we might be able to buy in the future, we might want to do nothing
            best = best.max(solve_internal(
                bp,
                cache,
                time_left - 1,
                step(bp, time_left, materials, machines),
                machines,
            ));
        }

        cache.insert(compress_state(time_left, materials, machines), best);
        best
    }
}

fn will_be_buyable(cost: Cost, machines: Machines) -> bool {
    cost.0
        .iter()
        .zip(machines.0.iter())
        .all(|(&cost, &income)| cost == 0 || income > 0)
}

pub fn solve_a() -> Result<i64> {
    let i = parse_input()?;
    Ok(i.par_iter()
        .map(|bp| solve_bp(bp, 24))
        .enumerate()
        .map(|(i, v)| (i + 1) as i64 * v)
        .sum::<i64>())
}

pub fn solve_b() -> Result<i64> {
    let i = parse_input()?;
    let i = &i[0..3.min(i.len())];
    Ok(i.par_iter().map(|bp| solve_bp(bp, 32)).product::<i64>())
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
