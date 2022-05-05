use super::{Key, Keys, Lookup, Program};
use crate::turing::{Action, Actions};
use std::cmp::min;
use std::fmt::{self, Display, Formatter};

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
    pub fn with_states(n: u8) -> Self {
        Self {
            n,
            program: vec![None; 2 * n as usize],
        }
    }

    pub fn insert<K, A>(&mut self, key: K, action: A)
    where
        K: Into<Key>,
        A: Into<Action>,
    {
        let key = key.into();
        self.program[key.idx()] = Some(action.into());
    }

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
        let number_of_states = min(program.n, 2); // TODO correct number
        let iterator = Actions::up_to(number_of_states);
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
