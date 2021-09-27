use std::convert::{From, Into};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Symbol {
    Blank,
    NonBlank,
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::Blank
    }
}

impl Default for &Symbol {
    fn default() -> Self {
        &Symbol::Blank
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum State {
    Halted,
    Stuck,
    Number(u8),
}

impl State {
    fn halted(&self) -> bool {
        matches!(self, State::Halted)
    }
}

pub struct Program {
    program: Vec<Action>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
        }
    }

    pub fn insert<K, A>(&mut self, key: K, action: A)
    where
        K: Into<Key>,
        A: Into<Action>,
    {
        let key = key.into();
        self.program.insert(key.idx(), action.into());
    }

    fn get(&self, key: &Key) -> Option<&Action> {
        self.program.get(key.idx())
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Key {
    state: State,
    symbol: Symbol,
}

impl From<(State, Symbol)> for Key {
    fn from(key: (State, Symbol)) -> Self {
        Self {
            state: key.0,
            symbol: key.1,
        }
    }
}

impl Key {
    fn idx(&self) -> usize {
        match (self.state, self.symbol) {
            (State::Number(s), Symbol::Blank) => (2 * s) as usize,
            (State::Number(s), Symbol::NonBlank) => (2 * s + 1) as usize,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    Halt,
    Do {
        symbol: Symbol,
        direction: Direction,
        state: State,
    },
}

impl From<(Symbol, Direction, State)> for Action {
    fn from(action: (Symbol, Direction, State)) -> Self {
        Action::Do {
            symbol: action.0,
            direction: action.1,
            state: action.2,
        }
    }
}

struct Tape {
    right: Vec<Symbol>,
    left: Vec<Symbol>,
}

impl Tape {
    fn empty() -> Tape {
        Tape {
            right: Vec::new(),
            left: Vec::new(),
        }
    }
}

type Head = i128;

fn move_to(head: &Head, direction: &Direction) -> Head {
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

pub struct Machine {
    head: Head,
    tape: Tape,
    program: Program,
    state: State,
}

impl Machine {
    pub fn new(state: State, program: Program) -> Self {
        Self {
            head: 0i128,
            tape: Tape::empty(),
            program,
            state,
        }
    }

    fn step(&mut self) {
        if !self.state.halted() {
            let key = Key {
                state: self.state,
                symbol: self.tape[self.head],
            };
            match self.program.get(&key) {
                None => self.state = State::Stuck,
                Some(Action::Halt) => self.state = State::Halted,
                Some(Action::Do {
                    symbol,
                    direction,
                    state,
                }) => {
                    self.tape[self.head] = *symbol;
                    self.head = move_to(&self.head, direction);
                    self.state = *state;
                }
            }
        }
    }

    pub fn run(&mut self, maximum_steps: u128) -> u128 {
        let mut steps_taken: u128 = 0u128;
        while !self.state.halted() && steps_taken < maximum_steps {
            self.step();
            steps_taken += 1;
        }
        steps_taken
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_symbols_are_equal() {
        assert_eq!(Symbol::Blank, Symbol::Blank);
        assert_eq!(Symbol::NonBlank, Symbol::NonBlank);
    }

    #[test]
    fn distinct_symbols_are_distinct() {
        assert_ne!(Symbol::Blank, Symbol::NonBlank);
        assert_ne!(Symbol::NonBlank, Symbol::Blank);
    }

    #[test]
    fn same_direction_are_equal() {
        assert_eq!(Direction::Left, Direction::Left);
        assert_eq!(Direction::Right, Direction::Right);
    }

    #[test]
    fn distinct_direction_are_distinct() {
        assert_ne!(Direction::Left, Direction::Right);
        assert_ne!(Direction::Right, Direction::Left);
    }

    #[test]
    fn same_states_are_equal() {
        assert_eq!(State::Halted, State::Halted);
        assert_eq!(State::Stuck, State::Stuck);
        assert_eq!(State::Number(0u8), State::Number(0u8));
    }

    #[test]
    fn distinct_states_are_distinct() {
        assert_ne!(State::Halted, State::Stuck);
        assert_ne!(State::Halted, State::Number(0u8));
        assert_ne!(State::Stuck, State::Halted);
        assert_ne!(State::Stuck, State::Number(0u8));
        assert_ne!(State::Number(0u8), State::Halted);
        assert_ne!(State::Number(0u8), State::Stuck);
    }

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

    #[test]
    fn a_simple_machine_can_be_run() {
        let mut program: Program = Program::new();
        program.insert(
            (State::Number(0), Symbol::Blank),
            (Symbol::NonBlank, Direction::Right, State::Number(1)),
        );
        program.insert(
            (State::Number(0), Symbol::NonBlank),
            (Symbol::NonBlank, Direction::Left, State::Halted),
        );
        program.insert(
            (State::Number(1), Symbol::Blank),
            (Symbol::NonBlank, Direction::Left, State::Number(0)),
        );
        program.insert(
            (State::Number(1), Symbol::NonBlank),
            (Symbol::NonBlank, Direction::Right, State::Halted),
        );
        let start = State::Number(0);
        let mut machine = Machine::new(start, program);

        let steps = machine.run(10);

        assert_eq!(steps, 3);
    }
}
