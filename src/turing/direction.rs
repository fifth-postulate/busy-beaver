#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_direction_are_equal() {
        assert_eq!(Direction::Left, Direction::Left);
        assert_eq!(Direction::Right, Direction::Right);
    }

    #[test]
    fn distinct_direction_are_distinct() {
        assert_ne!(Direction::Left, Direction::Right);
        assert_ne!(Direction::Right, Direction::Left);
    }
}
