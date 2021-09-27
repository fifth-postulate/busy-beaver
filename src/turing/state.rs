#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum State {
    Halted,
    Stuck,
    Number(u8),
}

impl State {
    pub fn halted(&self) -> bool {
        matches!(self, State::Halted) || matches!(self, State::Stuck)
    }
}

struct States {
    maximum: u8,
    current: Option<State>,
}

impl States {
    fn up_to(maximum: u8) -> Self {
        Self {
            maximum,
            current: Some(State::Halted),
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
        assert_eq!(State::Stuck, State::Stuck);
        assert_eq!(State::Number(0u8), State::Number(0u8));
    }

    #[test]
    fn distinct_states_are_distinct() {
        assert_ne!(State::Halted, State::Stuck);
        assert_ne!(State::Halted, State::Number(0u8));
        assert_ne!(State::Stuck, State::Halted);
        assert_ne!(State::Stuck, State::Number(0u8));
        assert_ne!(State::Number(0u8), State::Halted);
        assert_ne!(State::Number(0u8), State::Stuck);
    }

    #[test]
    fn halted_and_stuck_are_halted_states() {
        assert!(State::Halted.halted());
        assert!(State::Stuck.halted());
    }

    #[test]
    fn states_up_to_contains_all_states_up_to_argument() {
        let actual: Vec<State> = States::up_to(2).collect();

        assert_eq!(
            vec![State::Halted, State::Number(0), State::Number(1)],
            actual
        );
    }
}
