/// Naive implementation of a `Tape`.
/// 
/// Keeps two vectors of symbols, representing all cells to the left and all cells to the right, and an offset into these vectors.
use super::Tape;
use crate::turing::{direction::Direction, symbol::Symbol};

/// A naive implementation of a `Tape`
#[derive(Debug)]
pub struct SimpleTape {
    head: Head,
    right: Vec<Symbol>,
    left: Vec<Symbol>,
}

pub type Head = i128;

impl SimpleTape {
    /// Create an empty tape, i.e. a tape with only blank symbols on it
    pub fn empty() -> Self {
        Self {
            head: 0,
            right: Vec::new(),
            left: Vec::new(),
        }
    }

    fn right_index(&self) -> usize {
        assert!(self.head >= 0i128);
        self.head as usize
    }

    fn left_index(&self) -> usize {
        assert!(self.head < 0i128);
        (-self.head - 1) as usize
    }
}

impl Tape for SimpleTape {
    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.head -= 1,
            Direction::Right => self.head += 1,
        }
    }

    fn read(&self) -> Symbol {
        if self.head >= 0i128 {
            *self.right.get(self.right_index()).unwrap_or_default()
        } else {
            *self.left.get(self.left_index()).unwrap_or_default()
        }
    }

    fn write(&mut self, symbol: Symbol) {
        if self.head >= 0i128 {
            let i = self.right_index();
            if i >= self.right.len() {
                self.right.insert(i, Symbol::Blank)
            }
            self.right[i] = symbol;
        } else {
            let i = self.left_index();
            if i >= self.left.len() {
                self.left.insert(i, Symbol::Blank)
            }
            self.left[i] = symbol
        }
    }

    fn count(&self, target: &Symbol) -> usize {
        self.left.iter().filter(|s| *s == target).count()
            + self.right.iter().filter(|s| *s == target).count()
    }
}

impl Clone for SimpleTape {
    fn clone(&self) -> Self {
        let left = self.left.to_vec();
        let right = self.right.to_vec();
        Self {
            head: self.head,
            left,
            right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tape_contains_blanks() {
        let tape = SimpleTape::empty();

        assert_eq!(tape.read(), Symbol::Blank);
    }

    #[test]
    fn tape_can_be_written_to() {
        let mut tape = SimpleTape::empty();

        tape.write(Symbol::NonBlank);

        assert_eq!(tape.read(), Symbol::NonBlank);
    }

    #[test]
    fn tape_can_count_symbols() {
        let mut tape = SimpleTape::empty();

        tape.write(Symbol::NonBlank);
        tape.move_to(&Direction::Right);
        tape.write(Symbol::NonBlank);

        assert_eq!(tape.count(&Symbol::NonBlank), 2usize);
    }
}
