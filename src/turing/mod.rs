//! Allows the simulation of Turing machines with various characteristics.
//!
//! A [Turing machine](https://en.wikipedia.org/wiki/Turing_machine) is a
//! > mathematical model of computation describing an abstract machine that manipulates symbols on a strip of tape according to a table of rules.
mod direction;
mod machine;
mod program;
mod state;
mod symbol;
mod tape;

pub use direction::{Direction, Directions};
pub use machine::{Assessment, Details, Machine, Progress};
pub use program::{
    Action, Actions, CompleteProgram, CompletePrograms as Programs, IncompleteProgram, Key, Keys,
    Lookup, Program,
};
pub use state::{State, States};
pub use symbol::{Symbol, Symbols};
pub use tape::{CompoundTape, SimpleTape, Tape};
