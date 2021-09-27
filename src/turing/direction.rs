#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

struct Directions {
    current: Option<Direction>,
}

impl Directions {
    fn all() -> Self {
        Self {
            current: Some(Direction::Left),
        }
    }
}

impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.current;
        self.current = match item {
            Some(Direction::Left) => Some(Direction::Right),
            _ => None,
        };
        item
    }
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

    #[test]
    fn directions_all_contains_all_directions() {
        let actual: Vec<Direction> = Directions::all().collect();

        assert_eq!(vec![Direction::Left, Direction::Right], actual);
    }
}
