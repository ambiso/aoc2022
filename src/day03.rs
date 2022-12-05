use crate::{error::Result, util::read_string};

fn item_set(s: &[u8]) -> u64 {
    let mut x: u64 = 0;
    for &v in s {
        let upper_a = v.wrapping_sub('A' as u8).wrapping_add(27);
        let lower_a = v.wrapping_sub('a' as u8).wrapping_add(1);
        let which = (v >> 5) & 1 == 1;
        x |= 1 << (which as u8 * lower_a + (!which) as u8 * upper_a);
    }
    x
}

pub fn solve_a() -> Result<u32> {
    let mut s = 0;
    for l in read_string("inputs/day03a")?.lines() {
        let l = l.as_bytes();
        let left = &l[..l.len()/2];
        let right = &l[l.len()/2..];
        let intersection = item_set(left) & item_set(right);
        s += intersection.trailing_zeros();
    }
    Ok(s)
}

pub fn solve_b() -> Result<u32> {
    let mut s = 0;
    let mut running_intersection = !0u64;
    let mut group_member_count = 0;
    for l in read_string("inputs/day03a")?.lines() {
        running_intersection &= item_set(l.as_bytes());
        group_member_count += 1;
        if group_member_count == 3 {
            s += running_intersection.trailing_zeros();
            running_intersection = !0u64;
            group_member_count = 0;
        }
    }
    Ok(s)
}