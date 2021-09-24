#[derive(Debug, PartialEq, Eq)]
enum Symbol {
    Blank,
    NonBlank,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

type State = u8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_symbols_are_equal() {
        assert_eq!(Symbol::Blank, Symbol::Blank);
        assert_eq!(Symbol::NonBlank, Symbol::NonBlank);
    }

    #[test]
    fn distinct_symbols_are_distinct() {
        assert_ne!(Symbol::Blank, Symbol::NonBlank);
        assert_ne!(Symbol::NonBlank, Symbol::Blank);
    }

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
