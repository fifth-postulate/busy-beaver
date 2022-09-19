/// The configuration a Turing machine is in provides a key to lookup in a progam
use crate::turing::state::{State, States};
use crate::turing::symbol::{Symbol, Symbols};
use cartesian::*;

/// The current configuration of Turing machine
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Key {
    /// The state the Turing machine is in
    pub state: State,
    /// The symbol that the tape head is reading
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
    /// Keys have an index.
    /// TODO describe why and how it is calculated
    pub fn idx(&self) -> usize {
        match (self.state, self.symbol) {
            (State::Number(s), Symbol::Blank) => (2 * s) as usize,
            (State::Number(s), Symbol::NonBlank) => (2 * s + 1) as usize,
            _ => 0,
        }
    }
}

/// Iterator for keys
pub struct Keys {
    iterator: Box<dyn Iterator<Item = Key>>,
}

impl Keys {
    /// Iterate through a number of keys up to a maximum
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
