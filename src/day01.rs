use crate::error::Result;
use crate::util::read_string;

fn insert(top_s: &mut [i64], s: i64) {
    top_s[0] = s;
    top_s.sort();
}

pub fn solve_a() -> Result<()> {
    let mut s = 0;
    let mut top_s = [0i64, 0i64, 0i64, 0i64];
    for l in read_string("inputs/day01a")?.lines() {
        if l == "" {
            insert(&mut top_s, s);
            s = 0;
        } else {
            s += l.parse::<i64>()?;
        }
    }
    insert(&mut top_s, s);
    println!("{}", top_s[0]);
    println!("{}", top_s[1..].iter().sum::<i64>());
    println!("{:?}", top_s.as_slice());
    Ok(())
}
