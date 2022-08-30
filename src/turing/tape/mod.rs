mod simple;

use crate::turing::{direction::Direction, symbol::Symbol};
pub use simple::SimpleTape;

pub trait Tape {
    fn move_to(&mut self, direction: &Direction);
    fn read(&self) -> &Symbol;
    fn write(&mut self, symbol: Symbol);
    fn count(&self, target: &Symbol) -> usize;
}
