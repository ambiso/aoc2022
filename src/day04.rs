use crate::{error::Result, util::read_string};

pub fn solve_a() -> Result<(i32, i32)> {
    let mut s1 = 0;
    let mut s2 = 0;
    for pair in read_string("inputs/day04a")?.lines() {
        let mut i = pair.split(',');
        let a = i.next().unwrap();
        let b = i.next().unwrap();
        let mut i = a.split('-');
        let a_lo = i.next().unwrap().parse::<u64>()?;
        let a_hi = i.next().unwrap().parse::<u64>()?;
        let mut i = b.split('-');
        let b_lo = i.next().unwrap().parse::<u64>()?;
        let b_hi = i.next().unwrap().parse::<u64>()?;

        if (a_lo <= b_lo && b_hi <= a_hi) || (b_lo <= a_lo && a_hi <= b_hi) {
            s1 += 1;
        }
        if (a_lo <= b_lo && b_hi <= a_hi)
            || (b_lo <= a_lo && a_hi <= b_hi)
            || (a_hi >= b_lo && a_hi <= b_hi)
            || (a_lo >= b_lo && a_lo <= b_hi)
            || (b_hi >= a_lo && b_hi <= a_hi)
            || (b_lo >= a_lo && b_lo <= a_hi)
        {
            s2 += 1;
        }
    }
    Ok((s1, s2))
}
