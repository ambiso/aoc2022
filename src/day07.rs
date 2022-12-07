use std::collections::HashMap;

use regex::Regex;

use crate::{error::Result, util::read_string};

pub fn solve_a() -> Result<usize> {
    let mut tree = HashMap::new();
    let mut cur_dir = Vec::new();

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    for cmd in read_string("inputs/day07a")?.lines() {
        
    }

    Ok(1)
}

// pub fn solve_b() -> Result<usize> {
// }
