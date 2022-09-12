use super::Tape;
use crate::turing::{direction::Direction, symbol::Symbol};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Occurrence {
    Infinite,
    Finite(usize),
}

impl Occurrence {
    fn increment(&self) -> Self {
        match self {
            Occurrence::Infinite => Occurrence::Infinite,
            Occurrence::Finite(n) => Occurrence::Finite(n + 1),
        }
    }

    fn decrement(&self) -> Self {
        match self {
            Occurrence::Infinite => Occurrence::Infinite,
            Occurrence::Finite(n) if *n > 0 => Occurrence::Finite(n - 1),
            Occurrence::Finite(0) => Occurrence::Finite(0), // TODO is this the best model
            _ => panic!("this should not have happened"),
        }
    }

    fn count(&self) -> usize {
        match self {
            Occurrence::Infinite => usize::default(), // TODO is this the best model
            Occurrence::Finite(n) => *n,
        }
    }
}

impl PartialOrd for Occurrence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Occurrence {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Occurrence::Infinite, Occurrence::Infinite) => Ordering::Equal,
            (Occurrence::Finite(_), Occurrence::Infinite) => Ordering::Less,
            (Occurrence::Infinite, Occurrence::Finite(_)) => Ordering::Greater,
            (Occurrence::Finite(l), Occurrence::Finite(r)) => l.cmp(r),
        }
    }
}

#[derive(Debug)]
pub struct CompoundTape {
    right: Vec<(Symbol, Occurrence)>,
    left: Vec<(Symbol, Occurrence)>,
}

impl CompoundTape {
    pub fn empty() -> Self {
        Self {
            right: vec![(Symbol::Blank, Occurrence::Infinite)],
            left: vec![(Symbol::Blank, Occurrence::Infinite)],
        }
    }
}

impl Tape for CompoundTape {
    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => {
                let mut p = self.left.pop().unwrap(/* safe because of the sentinel value */);
                let symbol = p.0;
                if p.1 > Occurrence::Finite(1) {
                    p.1 = p.1.decrement();
                    self.left.push(p);
                }
                let mut q = self.right.pop().unwrap(/* safe because of the sentinel value */);
                if q.0 == symbol {
                    q.1 = q.1.increment();
                    self.right.push(q);
                } else {
                    self.right.push(q);
                    self.right.push((symbol, Occurrence::Finite(1)));
                }
            }
            Direction::Right => {
                let mut p = self.right.pop().unwrap(/* safe because of the sentinel value */);
                let symbol = p.0;
                if p.1 > Occurrence::Finite(1) {
                    p.1 = p.1.decrement();
                    self.right.push(p);
                }
                let mut q = self.left.pop().unwrap(/* safe because of the sentinel value */);
                if q.0 == symbol {
                    q.1 = q.1.increment();
                    self.left.push(q);
                } else {
                    self.left.push(q);
                    self.left.push((symbol, Occurrence::Finite(1)));
                }
            }
        };
    }

    fn read(&self) -> Symbol {
        self.right.last().map(|p| p.0).unwrap_or_default()
    }

    fn write(&mut self, symbol: Symbol) {
        if let Some((s, o)) = self.right.last() {
            if *s != symbol {
                if *o > Occurrence::Finite(1) {
                    let p = self.right.last_mut().unwrap(/* safe because of the outer guard */);
                    p.1.decrement();
                    self.right.push((symbol, Occurrence::Finite(1)));
                } else {
                    // o == Occurrence::Finite(1)
                    self.right.pop();
                    let p = self.right.last_mut().unwrap(/* safe because sentinel value */);
                    if p.0 == symbol {
                        p.1.increment();
                    } else {
                        self.right.push((symbol, Occurrence::Finite(1)));
                    }
                }
            }
        }
    }

    fn count(&self, target: &Symbol) -> usize {
        self.left
            .iter()
            .filter(|s| s.0 == *target)
            .map(|s| s.1.count())
            .sum::<usize>()
            + self
                .right
                .iter()
                .filter(|s| s.0 == *target)
                .map(|s| s.1.count())
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tape_contains_blanks() {
        let tape = CompoundTape::empty();

        assert_eq!(tape.read(), Symbol::Blank);
    }

    #[test]
    fn tape_can_be_written_to() {
        let mut tape = CompoundTape::empty();

        tape.write(Symbol::NonBlank);

        assert_eq!(tape.read(), Symbol::NonBlank);
    }

    #[test]
    fn tape_can_count_symbols() {
        let mut tape = CompoundTape::empty();

        tape.write(Symbol::NonBlank);
        tape.move_to(&Direction::Right);
        tape.write(Symbol::NonBlank);

        assert_eq!(tape.count(&Symbol::NonBlank), 2usize);
    }

    #[test]
    fn tape_can_move_efficiently() {
        let mut tape = CompoundTape::empty();

        for _ in 0..10 {
            tape.write(Symbol::NonBlank);
            tape.move_to(&Direction::Right);
        }

        assert_eq!(tape.count(&Symbol::NonBlank), 10usize);
    }
}
