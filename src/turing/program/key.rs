use crate::turing::state::{State, States};
use crate::turing::symbol::{Symbol, Symbols};
use cartesian::*;

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

impl From<usize> for Key {
    fn from(index: usize) -> Self {
        let state: State = index.into();
        let symbol: Symbol = index.into();
        Self { state, symbol }
    }
}

impl Key {
    pub fn idx(&self) -> usize {
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
}
