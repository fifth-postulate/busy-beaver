use super::{Key, Keys};
use super::{Lookup, Program};
use crate::turing::Action;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct IncompleteProgram {
    n: u8,
    program: Vec<Action>,
}

impl Program for IncompleteProgram {
    fn lookup(&self, key: &Key) -> Lookup {
        let idx = key.idx();
        match self.program.get(idx) {
            Some(action) => Lookup::Determined(*action),
            None => {
                if idx < (2 * self.n).into() {
                    Lookup::Indeterminate
                } else {
                    Lookup::Unknown
                }
            }
        }
    }
}

impl IncompleteProgram {
    pub fn with_states(n: u8) -> Self {
        Self {
            n,
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

    pub fn extentions(&self) -> Extention {
        Extention::of(self.clone())
    }
}

impl Clone for IncompleteProgram {
    fn clone(&self) -> Self {
        let mut program = IncompleteProgram::with_states(self.n);
        self.program
            .iter()
            .enumerate()
            .for_each(|(index, action)| program.insert(index, *action));

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

pub struct Extention {
    start: IncompleteProgram,
}

impl Extention {
    fn of(program: IncompleteProgram) -> Self {
        Self { start: program }
    }
}

impl Iterator for Extention {
    type Item = IncompleteProgram;

    fn next(&mut self) -> Option<Self::Item> {
        let program = self.start.clone();
        None // TODO iterate over all extentions
    }
}
