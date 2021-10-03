use crate::turing::direction::{Direction, Directions};
use crate::turing::state::{State, States};
use crate::turing::symbol::{Symbol, Symbols};
use cartesian::*;
use std::convert::{From, Into};
use std::iter::once;
use std::fmt;
use std::fmt::{Display, Formatter, Error};

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

impl Display for Program {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        let n = (self.program.len() / 2) as u8; // We are assume only complete programs
        let actions: Vec<String> = Keys::up_to(n)
            .map(|k| self.get(&k))
            .map(|ao| ao.map(|a| a.to_string() ).unwrap_or("???".to_string()))
            .collect();
        formatter.write_str(&actions.join(" "))
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

pub struct Keys {
    iterator: Box<dyn Iterator<Item = Key>>,
}

impl Keys {
    pub fn up_to(maximum: u8) -> Self {
        let iterator = cartesian!(States::non_halted_up_to(maximum), Symbols::all()).map(|tuple| {
            let key: Key = tuple.into();
            key
        });
        Self {
            iterator: Box::new(iterator),
        }
    }
}

impl Iterator for Keys {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
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

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Action::Halt => write!(f, "  H"),
            Action::Do { symbol, direction, state} => write!(f, "{}{}{}", symbol, direction, state),
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
    fn keys_up_to_contain_all_keys_up_to_maximum() {
        let actual: Vec<Key> = Keys::up_to(2).collect();
        let expected: Vec<Key> = vec![
            (State::Number(0), Symbol::Blank).into(),
            (State::Number(0), Symbol::NonBlank).into(),
            (State::Number(1), Symbol::Blank).into(),
            (State::Number(1), Symbol::NonBlank).into(),
        ];

        assert_eq!(expected, actual)
    }
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
