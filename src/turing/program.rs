use crate::turing::direction::Direction;
use crate::turing::state::State;
use crate::turing::symbol::Symbol;
use std::convert::{From, Into};

pub struct Program {
    program: Vec<Action>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
        }
    }

    pub fn insert<K, A>(&mut self, key: K, action: A)
    where
        K: Into<Key>,
        A: Into<Action>,
    {
        let key = key.into();
        self.program.insert(key.idx(), action.into());
    }

    pub fn get(&self, key: &Key) -> Option<&Action> {
        self.program.get(key.idx())
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Key {
    pub state: State,
    pub symbol: Symbol,
}

impl From<(State, Symbol)> for Key {
    fn from(key: (State, Symbol)) -> Self {
        Self {
            state: key.0,
            symbol: key.1,
        }
    }
}

impl Key {
    fn idx(&self) -> usize {
        match (self.state, self.symbol) {
            (State::Number(s), Symbol::Blank) => (2 * s) as usize,
            (State::Number(s), Symbol::NonBlank) => (2 * s + 1) as usize,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    Halt,
    Do {
        symbol: Symbol,
        direction: Direction,
        state: State,
    },
}

impl From<(Symbol, Direction, State)> for Action {
    fn from(action: (Symbol, Direction, State)) -> Self {
        Action::Do {
            symbol: action.0,
            direction: action.1,
            state: action.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo() {
        assert_eq!(1 + 1, 2);
    }
}
