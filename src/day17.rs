use std::{collections::VecDeque, fmt::Debug};

use crate::error::Result;

enum Collision {
    None,
    Rock,
    Wall,
    Floor,
}

#[derive(Eq, Clone, Hash)]
struct Map {
    m: VecDeque<[bool; 7]>,
    height_offset: usize,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for y in (0..self.m.len()).rev() {
            write!(f, "{: >8} ", y + self.height_offset)?;
            for v in self.m[y] {
                write!(f, "{}", if v { "#" } else { "." })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m
    }
}

impl Map {
    fn new() -> Self {
        Self {
            m: VecDeque::new(),
            height_offset: 0,
        }
    }
    fn get(&self, pos: (i64, i64)) -> bool {
        self.m
            .get(pos.1 as usize - self.height_offset)
            .map(|line| line[pos.0 as usize])
            .unwrap_or(false)
    }

    fn insert(&mut self, pos: (i64, i64)) {
        let max_height = self.m.len() as i64 + self.height_offset as i64 - 1;
        if pos.1 > max_height {
            let missing = (pos.1 - max_height) as usize;
            self.m.extend([[false; 7]].iter().cycle().take(missing));
        }
        self.m[pos.1 as usize - self.height_offset][pos.0 as usize] = true;
    }

    fn clear_lines(&mut self, height: i64) {
        let cleared = height as usize - self.height_offset;
        self.m.drain(0..cleared);
        self.height_offset += cleared;
    }

    fn height(&self) -> i64 {
        (self.m.len() + self.height_offset) as i64
    }
}

fn check_collision(rock: &[(i64, i64)], map: &Map, offset: (i64, i64)) -> Collision {
    for subrock in rock {
        let (x, y) = (subrock.0 + offset.0, subrock.1 + offset.1);
        if x < 0 || x >= 7 {
            return Collision::Wall;
        }
        if y < 0 {
            return Collision::Floor;
        }
        if map.get((x, y)) {
            return Collision::Rock;
        }
    }
    return Collision::None;
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    m: Map,
    rock_id: usize,
    move_id: usize,
}

pub fn solve_a() -> Result<i64> {
    let mut map = Map::new();

    let rock_formations: [&[(i64, i64)]; 5] = [
        &[(2i64, 0), (3, 0), (4, 0), (5, 0)],
        &[(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
        &[(4, 2), (4, 1), (2, 0), (3, 0), (4, 0)],
        &[(2, 0), (2, 1), (2, 2), (2, 3)],
        &[(2, 0), (3, 0), (2, 1), (3, 1)],
    ];

    let input = std::fs::read("inputs/day17a")?;
    let mut movements = input.trim_ascii().iter().cycle();

    let mut highest_pos = -1i64;

    for (rock_num, rock) in rock_formations.iter().cycle().enumerate() {
        let mut rock_pos = (0, highest_pos + 4);
        loop {
            // "spawn" rock
            // move instruction
            let movement = movements.next().unwrap();
            let m = match *movement as char {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };
            let move_intent = (rock_pos.0 + m, rock_pos.1);
            rock_pos = match check_collision(rock, &map, move_intent) {
                Collision::None => move_intent,
                Collision::Wall | Collision::Rock => rock_pos,
                Collision::Floor => unreachable!(),
            };
            // move down
            let move_intent = (rock_pos.0, rock_pos.1 - 1);
            rock_pos = match check_collision(rock, &map, move_intent) {
                Collision::None => move_intent,
                Collision::Rock | Collision::Floor => {
                    // solidify
                    for e in *rock {
                        let pos = (e.0 + rock_pos.0, e.1 + rock_pos.1);
                        highest_pos = highest_pos.max(pos.1);
                        map.insert(pos);
                        if (0..7).all(|x| map.get((x, pos.1))) {
                            map.clear_lines(pos.1);
                        }
                    }
                    break;
                }
                Collision::Wall => unreachable!(),
            };
        }
        // println!("BOARD");
        // for y in (0..=highest_pos).rev() {
        //     for x in 0..7 {
        //         if map.contains(&(x, y)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }
        if rock_num + 1 == 2022 {
            return Ok(highest_pos + 1);
        }
    }
    unreachable!()
}

#[derive(Debug)]
pub struct CycleResult {
    length: i64,
    offset: i64,
}

pub fn floyd<St: Eq + Clone>(f: impl Fn(St) -> St, x0: St) -> CycleResult {
    let mut tortoise = f(x0.clone());
    let mut hare = f(f(x0.clone()));

    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(f(hare));
    }

    let mut mu = 0;
    tortoise = x0.clone();

    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(hare);
        mu += 1;
    }

    let mut lam = 1;
    hare = f(tortoise.clone());
    while tortoise != hare {
        hare = f(hare);
        lam += 1
    }

    CycleResult {
        length: lam,
        offset: mu,
    }
}

pub fn solve_b() -> Result<i64> {
    let rock_formations: [&[(i64, i64)]; 5] = [
        &[(2i64, 0), (3, 0), (4, 0), (5, 0)],
        &[(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
        &[(4, 2), (4, 1), (2, 0), (3, 0), (4, 0)],
        &[(2, 0), (2, 1), (2, 2), (2, 3)],
        &[(2, 0), (3, 0), (2, 1), (3, 1)],
    ];

    let input = std::fs::read("inputs/day17a")?;
    let movements = input.trim_ascii();

    let step_fn = |mut st: State| {
        let map = &mut st.m;
        let rock = rock_formations[st.rock_id];
        let mut rock_pos = (
            0,
            (if map.m.len() == 0 {
                3
            } else {
                map.height() + 3
            }) as i64,
        );
        loop {
            let m = match movements[st.move_id] as char {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };
            st.move_id += 1;
            st.move_id %= movements.len();
            let move_intent = (rock_pos.0 + m, rock_pos.1);
            rock_pos = match check_collision(rock, &map, move_intent) {
                Collision::None => move_intent,
                Collision::Wall | Collision::Rock => rock_pos,
                Collision::Floor => unreachable!(),
            };
            // move down
            let move_intent = (rock_pos.0, rock_pos.1 - 1);
            rock_pos = match check_collision(rock, &map, move_intent) {
                Collision::None => move_intent,
                Collision::Rock | Collision::Floor => {
                    // solidify
                    for e in rock {
                        let pos = (e.0 + rock_pos.0, e.1 + rock_pos.1);
                        map.insert(pos);
                        if (0..7).all(|x| map.get((x, pos.1))) {
                            map.clear_lines(pos.1);
                        }
                    }
                    break;
                }
                Collision::Wall => unreachable!(),
            };
        }
        st.rock_id += 1;
        st.rock_id %= rock_formations.len();
        st
    };

    let x0 = State {
        m: Map::new(),
        rock_id: 0,
        move_id: 0,
    };
    let cycle = floyd(step_fn, x0.clone());

    let mut s = x0;
    for _ in 0..cycle.offset {
        s = step_fn(s);
    }

    let mut s2 = s.clone();
    for _ in 0..cycle.length {
        s2 = step_fn(s2);
    }

    assert_eq!(s, s2); // only the height_offset should change

    let n = 1000000000000;
    let skipped_cycles = (n - cycle.offset) / cycle.length;
    let skipped_iters = skipped_cycles * cycle.length;

    let height_difference = (s2.m.height() - s.m.height()) as i64;
    let move_id_diff = s2.move_id as i64 - s.move_id as i64;

    s.m.height_offset += (height_difference * skipped_cycles) as usize;
    s.move_id = (s.move_id as i64 + move_id_diff * skipped_cycles).rem_euclid(movements.len() as i64) as usize;
    s.rock_id = (s.rock_id + skipped_iters as usize) % rock_formations.len();

    let new_iters = cycle.offset + skipped_cycles * cycle.length;

    for _ in new_iters..n {
        s = step_fn(s);
    }

    Ok(s.m.height())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 3232);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_a().unwrap(), 1585632183915);
    }
}
