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
}
