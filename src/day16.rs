use crate::error::Result;
pub fn solve_a() -> Result<i64> {
    // substructure: f(where I am, how much time is left, values of the (modified) graph)
    Ok(0)
}

pub fn solve_b() -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 5256611);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 26686);
    }
}
