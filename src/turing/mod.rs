mod direction;
mod program;
mod state;
mod symbol;
mod tape;

pub use direction::{Direction, Directions};
pub use program::{
    Action, Actions, Key, Keys, Lookup, NaiveProgram, NaivePrograms as Programs, Program,
};
pub use state::{State, States};
pub use symbol::{Symbol, Symbols};
pub use tape::{move_to, Head, Tape};

pub struct Machine<'a> {
    head: Head,
    tape: Tape,
    program: &'a dyn Program,
    state: State,
}

impl<'a> Machine<'a> {
    pub fn new(program: &'a dyn Program) -> Self {
        Self {
            head: 0i128,
            tape: Tape::empty(),
            state: State::Number(0),
            program,
        }
    }

    pub fn with_state(state: State, program: &'a dyn Program) -> Self {
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
            match self.program.lookup(&key) {
                Lookup::Unknown => self.state = State::Stuck,
                Lookup::Indeterminate => self.state = State::Stuck,
                Lookup::Determined(Action::Halt) => self.state = State::Halted,
                Lookup::Determined(Action::Do {
                    symbol,
                    direction,
                    state,
                }) => {
                    self.tape[self.head] = symbol;
                    self.head = move_to(&direction, &self.head);
                    self.state = state;
                }
            }
        }
    }

    pub fn run(&mut self, maximum_steps: u128) -> Assessment {
        let mut steps_taken: u128 = 0u128;
        while !self.state.halted() && steps_taken < maximum_steps {
            self.step();
            steps_taken += 1;
        }
        if self.state.halted() {
            Assessment::HaltedIn(Details {
                steps: steps_taken,
                score: self.tape.count(&Symbol::NonBlank),
            })
        } else {
            Assessment::NotHalted
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Assessment {
    HaltedIn(Details),
    NotHalted,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Details {
    pub steps: u128,
    pub score: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_simple_machine_can_be_run() {
        let mut program: NaiveProgram = NaiveProgram::new();
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
        let mut machine = Machine::with_state(start, &program);

        let steps = machine.run(10);

        assert_eq!(steps, Assessment::HaltedIn(Details { steps: 3, score: 2 }));
    }
}
