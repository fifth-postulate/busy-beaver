//! The tape of a Turing machine
//!
//! A tape can be writen to, can be read and can be moved in either direction.
mod compound;
mod simple;

use crate::turing::{direction::Direction, symbol::Symbol};
pub use compound::CompoundTape;
pub use simple::SimpleTape;

/// Characteristics of a tape.
pub trait Tape {
    /// A tape can be moved in either direction.
    fn move_to(&mut self, direction: &Direction);
    /// read the cell the tape head currently points to.
    fn read(&self) -> Symbol;
    /// write a symbol to the cell the tape head currently points to.
    fn write(&mut self, symbol: Symbol);
    /// count the number of occurences of the target symbol on the tape.
    fn count(&self, target: &Symbol) -> usize;
}
