use crate::error::Result;

pub fn solve_a() -> Result<i64> {
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
        assert_eq!(solve_a().unwrap(), 3877);
    }
    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 982);
    }
}
