use crate::turing::direction::Direction;
use crate::turing::symbol::Symbol;
use std::ops::{Index, IndexMut};

pub struct Tape {
    right: Vec<Symbol>,
    left: Vec<Symbol>,
}

impl Tape {
    pub fn empty() -> Tape {
        Tape {
            right: Vec::new(),
            left: Vec::new(),
        }
    }
}

pub type Head = i128;

pub fn move_to(direction: &Direction, head: &Head) -> Head {
    match direction {
        Direction::Left => head - 1,
        Direction::Right => head + 1,
    }
}

impl Index<Head> for Tape {
    type Output = Symbol;

    fn index(&self, index: Head) -> &Self::Output {
        if index >= 0i128 {
            self.right.get(index as usize).unwrap_or_default()
        } else {
            self.left.get((-index - 1) as usize).unwrap_or_default()
        }
    }
}

impl IndexMut<Head> for Tape {
    fn index_mut(&mut self, index: Head) -> &mut Self::Output {
        if index >= 0i128 {
            let i = index as usize;
            if i >= self.right.len() {
                self.right.insert(i, Symbol::Blank)
            }
            self.right.index_mut(index as usize)
        } else {
            let i = (-index - 1) as usize;
            if i >= self.left.len() {
                self.left.insert(i, Symbol::Blank)
            }
            self.left.index_mut(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tape_contains_blanks() {
        let tape = Tape::empty();
        let head: Head = 0i128;

        assert_eq!(tape[head], Symbol::Blank);
    }

    #[test]
    fn tape_can_be_written_to() {
        let mut tape = Tape::empty();
        let head: Head = 0i128;

        tape[head] = Symbol::NonBlank;

        assert_eq!(tape[head], Symbol::NonBlank);
    }
}
