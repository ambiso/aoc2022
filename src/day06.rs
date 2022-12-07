use crate::error::Result;

fn solve<const N: usize>() -> Result<usize> {
    let f = std::fs::read("inputs/day06a")?;
    let n = 4;
    let (x, _) = f
        .windows(N)
        .enumerate()
        .find(|(_, x)| {
            for i in 0..x.len() {
                for j in i+1..x.len() {
                    if x[i] == x[j] {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap();
    Ok(x + N)
}

pub fn solve_a() -> Result<usize> {
    solve::<4>()
}

pub fn solve_b() -> Result<usize> {
    solve::<14>()
}
