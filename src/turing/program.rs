use crate::turing::direction::{Direction, Directions};
use crate::turing::state::{State, States};
use crate::turing::symbol::{Symbol, Symbols};
use cartesian::*;
use std::convert::{From, Into};
use std::iter::once;

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

pub struct Actions {
    iterator: Box<dyn Iterator<Item = Action>>,
}

impl Actions {
    pub fn up_to(maximum: u8) -> Self {
        let iterator =
            once(Action::Halt).chain(States::non_halted_up_to(maximum).flat_map(|state| {
                cartesian!(Symbols::all(), Directions::all()).map(move |tuple| {
                    let action: Action = (tuple.0, tuple.1, state).into();
                    action
                })
            }));
        Self {
            iterator: Box::new(iterator),
        }
    }
}

impl Iterator for Actions {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions_up_to_contain_all_actions_up_to_maximum() {
        let actual: Vec<Action> = Actions::up_to(1).collect();

        assert_eq!(
            vec![
                Action::Halt,
                (Symbol::Blank, Direction::Left, State::Number(0)).into(),
                (Symbol::Blank, Direction::Right, State::Number(0)).into(),
                (Symbol::NonBlank, Direction::Left, State::Number(0)).into(),
                (Symbol::NonBlank, Direction::Right, State::Number(0)).into(),
            ],
            actual
        )
    }
}
