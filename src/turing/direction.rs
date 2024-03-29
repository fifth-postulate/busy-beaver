//! Directions the tape head can move in
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// The different directions the tape head can move in.
pub enum Direction {
    /// The tape head can move left
    Left,
    /// The tape head can move right
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseError::UnknownSymbol(input.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownSymbol(String),
}

/// An iterator for `Direction`s.
pub struct Directions {
    current: Option<Direction>,
}

impl Directions {
    /// Creates an iterator that iterates over all directions.
    pub fn all() -> Self {
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
    fn distinct_direction_are_not_equal() {
        assert_ne!(Direction::Left, Direction::Right);
        assert_ne!(Direction::Right, Direction::Left);
    }

    #[test]
    fn directions_can_be_parsed() {
        assert_eq!(Ok(Direction::Left), "L".parse());
        assert_eq!(Ok(Direction::Right), "R".parse());
    }

    #[test]
    fn directions_all_contains_all_directions() {
        let actual: Vec<Direction> = Directions::all().collect();

        assert_eq!(vec![Direction::Left, Direction::Right], actual);
    }
}
