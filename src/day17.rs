use std::collections::{HashSet, VecDeque};

use crate::error::Result;

enum Collision {
    None,
    Rock,
    Wall,
    Floor,
}

struct Map {
    m: VecDeque<[bool; 7]>,
    height_offset: usize,
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
            let missing = pos.1 as usize + 1 - max_height as usize;
            self.m.extend(vec![[false; 7]; missing]);
        }
        self.m[pos.1 as usize - self.height_offset][pos.0 as usize] = true;
    }

    fn clear_lines(&mut self, height: i64) {
        let cleared = height as usize-self.height_offset;
        self.m.drain(0..cleared);
        self.height_offset += cleared;
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
    for (rock_id, rock) in rock_formations.iter().cycle().enumerate() {
        let mut rock_pos = (0, highest_pos + 4);
        loop {
            // "spawn" rock
            // move instruction
            let m = match *movements.next().unwrap() as char {
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
        if rock_id + 1 == 2022 {
            return Ok(highest_pos+1);
        }
    }
    unreachable!()
}

pub fn solve_b() -> Result<i64> {
    Ok(0)
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
        assert_eq!(solve_b().unwrap(), 26686);
    }
}
