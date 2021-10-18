use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum State {
    Halted,
    Number(u8),
}

impl State {
    pub fn halted(&self) -> bool {
        matches!(self, State::Halted)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            State::Halted => write!(f, "H"),
            State::Number(n) => write!(f, "{}", n),
        }
    }
}

impl From<usize> for State {
    fn from(index: usize) -> Self {
        let n = index / 2;
        State::Number(n as u8)
    }
}

impl FromStr for State {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "H" => Ok(State::Halted),
            _ => {
                let index = input.parse::<u8>();
                index
                    .map(State::Number)
                    .map_err(|_e| ParseError::UnknownSymbol(input.to_owned()))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownSymbol(String),
}

pub struct States {
    maximum: u8,
    current: Option<State>,
}

impl States {
    pub fn up_to(maximum: u8) -> Self {
        Self {
            maximum,
            current: Some(State::Halted),
        }
    }

    pub fn non_halted_up_to(maximum: u8) -> Self {
        Self {
            maximum,
            current: Some(State::Number(0)),
        }
    }
}

impl Iterator for States {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.current;
        self.current = match item {
            Some(State::Halted) => {
                if self.maximum > 0 {
                    Some(State::Number(0))
                } else {
                    None
                }
            }
            Some(State::Number(m)) => {
                if m + 1 < self.maximum {
                    Some(State::Number(m + 1))
                } else {
                    None
                }
            }
            _ => None,
        };
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_states_are_equal() {
        assert_eq!(State::Halted, State::Halted);
        assert_eq!(State::Number(0u8), State::Number(0u8));
    }

    #[test]
    fn distinct_states_are_distinct() {
        assert_ne!(State::Halted, State::Number(0u8));
        assert_ne!(State::Number(0u8), State::Halted);
    }

    #[test]
    fn halted_and_stuck_are_halted_states() {
        assert!(State::Halted.halted());
    }

    #[test]
    fn states_can_be_parsed() {
        assert_eq!(Ok(State::Halted), "H".parse());
        assert_eq!(Ok(State::Number(0)), "0".parse());
        assert_eq!(Ok(State::Number(1)), "1".parse());
    }

    #[test]
    fn states_up_to_contains_all_states_up_to_argument() {
        let actual: Vec<State> = States::up_to(2).collect();

        assert_eq!(
            vec![State::Halted, State::Number(0), State::Number(1)],
            actual
        );
    }

    #[test]
    fn non_halted_state_contain_all_non_halted_states_up_to_argument() {
        let actual: Vec<State> = States::non_halted_up_to(3).collect();

        assert_eq!(
            vec![State::Number(0), State::Number(1), State::Number(2)],
            actual
        );
    }
}
