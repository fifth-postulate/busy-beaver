use crate::turing::direction::Direction;
use crate::turing::symbol::Symbol;

pub struct Tape {
    head: Head,
    right: Vec<Symbol>,
    left: Vec<Symbol>,
}

pub type Head = i128;

impl Tape {
    pub fn empty() -> Tape {
        Tape {
            head: 0,
            right: Vec::new(),
            left: Vec::new(),
        }
    }

    pub fn count(&self, target: &Symbol) -> usize {
        self.left.iter().filter(|s| *s == target).count()
            + self.right.iter().filter(|s| *s == target).count()
    }

    pub fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.head -= 1,
            Direction::Right => self.head += 1,
        }
    }

    pub fn read(&self) -> &Symbol {
        if self.head >= 0i128 {
            self.right.get(self.head as usize).unwrap_or_default()
        } else {
            self.left.get((-self.head - 1) as usize).unwrap_or_default()
        }
    }

    pub fn write(&mut self, symbol: Symbol) {
        if self.head >= 0i128 {
            let i = self.head as usize;
            if i >= self.right.len() {
                self.right.insert(i, Symbol::Blank)
            }
            self.right[i] = symbol;
        } else {
            let i = (-self.head - 1) as usize;
            if i >= self.left.len() {
                self.left.insert(i, Symbol::Blank)
            }
            self.left[i] = symbol
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tape_contains_blanks() {
        let tape = Tape::empty();

        assert_eq!(*tape.read(), Symbol::Blank);
    }

    #[test]
    fn tape_can_be_written_to() {
        let mut tape = Tape::empty();

        tape.write(Symbol::NonBlank);

        assert_eq!(*tape.read(), Symbol::NonBlank);
    }

    #[test]
    fn tape_can_count_symbols() {
        let mut tape = Tape::empty();

        tape.write(Symbol::NonBlank);
        tape.move_to(&Direction::Right);
        tape.write(Symbol::NonBlank);

        assert_eq!(tape.count(&Symbol::NonBlank), 2usize);
    }
}
