//! A Program tells a Turing machine how to behave.

mod action;
mod complete;
mod incomplete;
mod key;

pub use action::{Action, Actions};
pub use complete::{CompleteProgram, CompletePrograms};
pub use incomplete::IncompleteProgram;
pub use key::{Key, Keys};
use std::convert::From;

/// The instructions for a Turing machine
pub trait Program {
    /// Lookup the action for the specific key
    fn lookup(&self, key: &Key) -> Lookup;

    /// Some program are abstract and represent multiple concrete programs. The multiplicity reflects how many concrete programs are represented by this program.
    fn multiplicity(&self) -> usize {
        1
    }
}

/// The result of looking up a certain key in a program.
pub enum Lookup {
    /// The key is not known to the program. A semantic error
    Unknown,
    /// The key is not known to the program. But the program can be extended with the lookup key
    Indeterminate,
    /// The determined action for this key
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
