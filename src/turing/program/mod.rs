mod action;
mod complete;
mod incomplete;
mod key;

pub use action::{Action, Actions};
pub use complete::{CompleteProgram, CompletePrograms};
pub use incomplete::IncompleteProgram;
pub use key::{Key, Keys};
use std::convert::From;

pub trait Program {
    fn lookup(&self, key: &Key) -> Lookup;

    fn multiplicity(&self) -> usize {
        1
    }
}

pub enum Lookup {
    Unknown,
    Indeterminate,
    Determined(Action),
}

impl From<Lookup> for Option<Action> {
    fn from(lookup: Lookup) -> Self {
        match lookup {
            Lookup::Determined(action) => Some(action),
            _ => None,
        }
    }
}
