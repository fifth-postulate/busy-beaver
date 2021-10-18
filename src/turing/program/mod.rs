mod action;
mod complete;
mod key;

pub use action::{Action, Actions};
pub use complete::{CompleteProgram, CompletePrograms};
pub use key::{Key, Keys};
use std::convert::From;

pub trait Program {
    fn lookup(&self, key: &Key) -> Lookup;
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
