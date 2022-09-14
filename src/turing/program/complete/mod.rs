//! A complete program
use super::action;
use super::action::{Action, Actions};
use super::{Key, Keys};
use super::{Lookup, Program};
use cartesian::*;
use std::convert::Into;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::IntoIterator;
use std::str::FromStr;

/// A complete program
#[derive(Debug, PartialEq, Eq)]
pub struct CompleteProgram {
    program: Vec<Action>,
}

impl Program for CompleteProgram {
    fn lookup(&self, key: &Key) -> Lookup {
        match self.program.get(key.idx()) {
            Some(action) => Lookup::Determined(*action),
            None => Lookup::Unknown,
        }
    }
}

impl CompleteProgram {
    /// Create an empty complete progam
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
        }
    }

    /// Insert an action for a certain key into the program
    pub fn insert<K, A>(&mut self, key: K, action: A)
    where
        K: Into<Key>,
        A: Into<Action>,
    {
        let key = key.into();
        self.program.insert(key.idx(), action.into());
    }
}

impl Default for CompleteProgram {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CompleteProgram {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let n = (self.program.len() / 2) as u8; // We assume only complete programs
        let actions: Vec<String> = Keys::up_to(n)
            .map(|k| self.lookup(&k))
            .map(|l| {
                let ao: Option<Action> = l.into();
                ao.map(|a| a.to_string())
                    .unwrap_or_else(|| "???".to_string())
            })
            .collect();
        formatter.write_str(&actions.join(" "))
    }
}

impl FromStr for CompleteProgram {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !input.is_empty() {
            let mut program = CompleteProgram::new();
            let mut action_index = 0;
            let mut index = 4 * action_index;
            while index < input.len() && (index + 3) <= input.len() {
                let action = input[index..(index + 3)]
                    .parse::<Action>()
                    .map_err(|error| {
                        ParseError::ActionProblem(ActionProblemDetail { index, error })
                    })?;
                let key: Key = action_index.into();
                program.insert(key, action);
                action_index += 1;
                index = 4 * action_index;
            }
            if action_index % 2 == 0 {
                Ok(program)
            } else {
                Err(ParseError::InsufficientActions(action_index))
            }
        } else {
            Err(ParseError::NoInput)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ActionProblem(ActionProblemDetail),
    InsufficientActions(usize),
    NoInput,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActionProblemDetail {
    index: usize,
    error: action::ParseError,
}

pub struct KeyActionIterator<'a> {
    index: usize,
    program: &'a CompleteProgram,
}

impl<'a> IntoIterator for &'a CompleteProgram {
    type Item = (Key, Action);
    type IntoIter = KeyActionIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        KeyActionIterator::new(self)
    }
}

impl<'a> KeyActionIterator<'a> {
    fn new(program: &'a CompleteProgram) -> Self {
        Self { index: 0, program }
    }
}

impl<'a> Iterator for KeyActionIterator<'a> {
    type Item = (Key, Action);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.program.program.len() {
            let key: Key = self.index.into();
            let lookup = self.program.lookup(&key);
            self.index += 1;
            let action: Option<Action> = lookup.into();
            action.map(|a| (key, a))
        } else {
            None
        }
    }
}

/// Iterator for complete progams
pub struct CompletePrograms {
    iterator: Box<dyn Iterator<Item = CompleteProgram>>,
}

macro_rules! actions {
    (1) => {
        cartesian!(Actions::up_to(1), Actions::up_to(1))
    };
    (2) => {
        cartesian!(
            Actions::up_to(2),
            Actions::up_to(2),
            Actions::up_to(2),
            Actions::up_to(2)
        )
    };
    (3) => {
        cartesian!(
            Actions::up_to(3),
            Actions::up_to(3),
            Actions::up_to(3),
            Actions::up_to(3),
            Actions::up_to(3),
            Actions::up_to(3)
        )
    };
    (4) => {
        cartesian!(
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4)
        )
    };
    (5) => {
        cartesian!(
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5)
        )
    };
}

macro_rules! tuple {
    (1) => {
        |tuple| vec![tuple.0, tuple.1]
    };
    (2) => {
        |tuple| vec![tuple.0, tuple.1, tuple.2, tuple.3]
    };
    (3) => {
        |tuple| vec![tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5]
    };
    (4) => {
        |tuple| {
            vec![
                tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7,
            ]
        }
    };
    (5) => {
        |tuple| {
            vec![
                tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
                tuple.9,
            ]
        }
    };
}

macro_rules! all_programs {
    ($n:tt, $fname:ident) => {
        fn $fname() -> CompletePrograms {
            let iterator = actions!($n).map(tuple!($n)).map(|actions| {
                let mut program = CompleteProgram::new();
                for (key, action) in Keys::up_to($n).zip(actions) {
                    program.insert(key, action);
                }
                program
            });
            CompletePrograms {
                iterator: Box::new(iterator),
            }
        }
    };
}

all_programs!(1, all1);
all_programs!(2, all2);
all_programs!(3, all3);
all_programs!(4, all4);
all_programs!(5, all5);

impl CompletePrograms {
    /// Create an iterator that iteratos through all complete programs of a certain number of states
    pub fn all(n: u8) -> Self {
        match n {
            1 => all1(),
            2 => all2(),
            3 => all3(),
            4 => all4(),
            5 => all5(),
            _ => panic!("it is unwise to go beyond 5"),
        }
    }
}

impl Iterator for CompletePrograms {
    type Item = CompleteProgram;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::turing::{Direction, State, Symbol};

    #[test]
    fn programs_can_be_parsed() {
        let mut expected = CompleteProgram::new();
        expected.insert(
            (State::Number(0), Symbol::Blank),
            (Symbol::NonBlank, Direction::Right, State::Number(1)),
        );
        expected.insert(
            (State::Number(0), Symbol::NonBlank),
            (Symbol::Blank, Direction::Right, State::Number(1)),
        );
        expected.insert(
            (State::Number(1), Symbol::Blank),
            (Symbol::NonBlank, Direction::Left, State::Number(1)),
        );
        expected.insert(
            (State::Number(1), Symbol::NonBlank),
            (Symbol::NonBlank, Direction::Right, State::Number(2)),
        );

        assert_eq!(Ok(expected), "1R1 0R1 1L1 1R2".parse())
    }

    #[test]
    fn key_action_iterator_can_be_iterated() {
        let program: CompleteProgram = "1R0 0R0".parse().unwrap();
        let expected: Vec<(Key, Action)> = vec![
            (
                (State::Number(0), Symbol::Blank).into(),
                (Symbol::NonBlank, Direction::Right, State::Number(0)).into(),
            ),
            (
                (State::Number(0), Symbol::NonBlank).into(),
                (Symbol::Blank, Direction::Right, State::Number(0)).into(),
            ),
        ];

        let actual: Vec<(Key, Action)> = KeyActionIterator::new(&program).collect();

        assert_eq!(expected, actual)
    }

    #[test]
    fn program_can_be_iterated() {
        let program: CompleteProgram = "1R0 0R0".parse().unwrap();
        let expected: Vec<(Key, Action)> = vec![
            (
                (State::Number(0), Symbol::Blank).into(),
                (Symbol::NonBlank, Direction::Right, State::Number(0)).into(),
            ),
            (
                (State::Number(0), Symbol::NonBlank).into(),
                (Symbol::Blank, Direction::Right, State::Number(0)).into(),
            ),
        ];

        let actual: Vec<(Key, Action)> = program.into_iter().collect();

        assert_eq!(expected, actual)
    }

    #[test]
    fn all1_contains_correct_number_of_programs() {
        let number_of_programs = all1().count();

        assert_eq!(number_of_programs, 25);
    }

    #[test]
    fn all2_contains_correct_number_of_programs() {
        let number_of_programs = all2().count();

        assert_eq!(number_of_programs, 6561);
    }
}
