//! Simulation a Turing machines with various characteristics.
//!
//! A [Turing machine](https://en.wikipedia.org/wiki/Turing_machine) is a
//! > mathematical model of computation describing an abstract machine that manipulates symbols on a strip of tape according to a table of rules.
//! 
//! A Turing machine consists of a two-sided infinite tape which is divided into a number of infinite number of cells.
//! There is a tape head that, at each instant in time, points to a single cell of the tape. The head can read the symbol that is written in the cell.
//! as well as write a symbol to the cell. The head can also move over the tape in either direction.
//! Each Turing machine can be in a finite number of states.
//! A Turing machine behavour is determined by its program. The program determines the action to take depending on the current key. The key is a combination
//! of the current state and the current symbol read from the tape. The action is a combination of a symbol to write to the tape, a direction to move the tape
//! head in and a state to transition to.
//! 
//! The following code demonstrates how to run Rado's example champion.
//! 
//! ```
//! # use busy_beaver::turing::{CompleteProgram, SimpleTape, Machine};
//! let program: CompleteProgram = "1L1 1R2 1R0 1L1 1R1 1LH".parse().unwrap();
//! let mut machine = Machine::new(SimpleTape::empty(), &program);
//! let assessment = machine.run(50_000_000);
//! println!("{:?}", assessment);
//! ```

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
