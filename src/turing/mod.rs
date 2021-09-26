use std::collections::HashMap;
use std::convert::{From, Into};
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Symbol {
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
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum State {
    Halt,
    Number(u8),
}

impl State {
    fn halted(&self) -> bool {
        matches!(self, State::Halt)
    }
}

struct Program {
    program: HashMap<Key, Action>,
}

impl Program {
    fn new() -> Self {
        Self {
            program: HashMap::new(),
        }
    }

    fn insert<K, A>(&mut self, key: K, action: A)
    where
        K: Into<Key>,
        A: Into<Action>,
    {
        self.program.insert(key.into(), action.into());
    }

    fn get(&self, key: &Key) -> Option<&Action> {
        self.program.get(key)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Key {
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Action {
    symbol: Symbol,
    direction: Direction,
    state: State,
}

impl From<(Symbol, Direction, State)> for Action {
    fn from(action: (Symbol, Direction, State)) -> Self {
        Self {
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
            self.left.get((-index) as usize).unwrap_or_default()
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
            let i = (-index) as usize;
            if i >= self.left.len() {
                self.left.insert(i, Symbol::Blank)
            }
            self.left.index_mut((-index) as usize)
        }
    }
}

struct Machine {
    head: Head,
    tape: Tape,
    program: Program,
    state: State,
}

impl Machine {
    fn new(start_state: State, program: Program) -> Self {
        Self {
            head: 0i128,
            tape: Tape::empty(),
            program: program,
            state: start_state,
        }
    }

    fn step(&mut self) {
        if !self.state.halted() {
            let key = Key {
                state: self.state,
                symbol: self.tape[self.head],
            };
            match self.program.get(&key) {
                None => self.state = State::Halt,
                Some(action) => {
                    self.tape[self.head] = action.symbol;
                    self.head = move_to(&self.head, &action.direction);
                    self.state = action.state;
                }
            }
        }
    }

    fn run(&mut self, maximum_steps: u128) -> u128 {
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
    fn distinct_states_are_distinct() {
        assert_ne!(State::Halt, State::Number(0u8));
        assert_ne!(State::Number(0u8), State::Halt);
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
            (Symbol::NonBlank, Direction::Left, State::Halt),
        );
        program.insert(
            (State::Number(1), Symbol::Blank),
            (Symbol::NonBlank, Direction::Left, State::Number(0)),
        );
        program.insert(
            (State::Number(1), Symbol::NonBlank),
            (Symbol::NonBlank, Direction::Right, State::Halt),
        );
        let start = State::Number(0);
        let mut machine = Machine::new(start, program);

        let steps = machine.run(10);

        assert_eq!(steps, 3);
    }
}
