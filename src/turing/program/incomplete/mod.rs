//! Incomplete programs
use super::{Key, Keys, Lookup, Program};
use crate::turing::{Action, Actions, State};
use std::cmp::{max, min};
use std::fmt::{self, Display, Formatter};

/// An incomplete program
#[derive(Debug, PartialEq, Eq)]
pub struct IncompleteProgram {
    n: u8,
    program: Vec<Option<Action>>,
}

impl Program for IncompleteProgram {
    fn lookup(&self, key: &Key) -> Lookup {
        let idx = key.idx();
        match self.program.get(idx) {
            Some(Some(action)) => Lookup::Determined(*action),
            Some(None) => Lookup::Indeterminate,
            None => Lookup::Unknown,
        }
    }

    fn multiplicity(&self) -> usize {
        self.program
            .iter()
            .filter(|action| action.is_none())
            .map(|_| (4 * self.n + 1) as usize)
            .product()
    }
}

impl IncompleteProgram {
    /// Create an incomplete program with a maximum number of states.
    pub fn with_states(n: u8) -> Self {
        Self {
            n,
            program: vec![None; 2 * n as usize],
        }
    }

    /// Insert an action for a certain key
    pub fn insert<K, A>(&mut self, key: K, action: A)
    where
        K: Into<Key>,
        A: Into<Action>,
    {
        let key = key.into();
        self.program[key.idx()] = Some(action.into());
    }

    /// return an iterator that extends this program in all sensible ways.
    pub fn extentions<K>(&self, key: K) -> Extentions
    where
        K: Into<Key>,
    {
        Extentions::of(self.clone(), key.into())
    }
}

impl Clone for IncompleteProgram {
    fn clone(&self) -> Self {
        let mut program = IncompleteProgram::with_states(self.n);
        self.program
            .iter()
            .enumerate()
            .filter(|(_, action)| action.is_some())
            .for_each(|(index, action)| program.insert(index, action.unwrap()));

        program
    }
}

impl Display for IncompleteProgram {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let actions: Vec<String> = Keys::up_to(self.n)
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

pub struct Extentions {
    key: Key,
    program: IncompleteProgram,
    iterator: Box<dyn Iterator<Item = Action>>,
}

impl Extentions {
    fn of(program: IncompleteProgram, key: Key) -> Self {
        let k: u8 = match key.state {
            State::Number(s) => s,
            State::Halted => 0, // Does not occur
        };
        let seen_state = program
            .program
            .iter()
            .enumerate()
            .filter(|(_, action)| action.is_some())
            .map(|(index, _)| index.into())
            .map(|state| match state {
                State::Number(s) => s,
                State::Halted => 0, // Does not occur
            })
            .max()
            .map(|s| max(s, k))
            .unwrap_or(k);
        let states_to_explore = min(program.n - 1, seen_state + 1);
        let iterator = Actions::up_to(states_to_explore + 1);
        Self {
            key,
            program,
            iterator: Box::new(iterator),
        }
    }
}

impl Iterator for Extentions {
    type Item = IncompleteProgram;

    fn next(&mut self) -> Option<Self::Item> {
        let mut program = self.program.clone();
        self.iterator.next().map(move |action| {
            program.insert(self.key, action);
            program
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::turing::{Direction, State, Symbol};

    #[test]
    fn incomplete_programs_can_be_displayed() {
        let mut program = IncompleteProgram::with_states(1);
        program.insert(
            (State::Number(0), Symbol::Blank),
            (Symbol::NonBlank, Direction::Right, State::Number(0)),
        );

        let actual = format!("{}", program);

        assert_eq!("1R0 ???", actual);
    }

    #[test]
    fn empty_incomplete_programs_can_be_extended() {
        let program = IncompleteProgram::with_states(2);

        let actual: Vec<String> = program
            .extentions((State::Number(0), Symbol::Blank))
            .map(|p| format!("{}", p))
            .collect();

        assert_eq!(
            vec![
                "  H ??? ??? ???",
                "0L0 ??? ??? ???",
                "0R0 ??? ??? ???",
                "1L0 ??? ??? ???",
                "1R0 ??? ??? ???",
                "0L1 ??? ??? ???",
                "0R1 ??? ??? ???",
                "1L1 ??? ??? ???",
                "1R1 ??? ??? ???",
            ],
            actual,
        )
    }
}
